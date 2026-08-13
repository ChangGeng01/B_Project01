//! 部门层级闭包读取契约与维护算法（A-04，02 计划 §4.8）。
//!
//! trait 签名逐字冻结，任何阶段不得改写。维护算法在本模块写成纯逻辑：
//! 部门新增、改父与停用三种写入，在同一事务内全量重写该部门为根的子树——
//! 先按 `ancestor_department_id` 删除该子树的既有行，再逐层插入，`depth`
//! 自零起，同一事务内一并维护 `departments.level_no`。
//! 不使用递归 CTE 做在线查询（基线第 3.10 节：附录 A.1 度量查询不得出现顺序扫描，
//! 闭包行已物化，读取只查 `department_closures` 单表）。
//!
//! SQL 的执行体落在 ep-adapter-db-pg 的 `platform_core` 仓储目录内，
//! 本模块不出现任何数据库专有语法。

use ep_foundation::error::AppError;
use ep_foundation::id::marker::{Department, LegalEntity};
use ep_foundation::id::Id;
use ep_foundation::port::tx::Tx;

/// 部门闭包查询。阶段 4 的部门闭包编译经 [`DepartmentClosureQuery::descendant_ids`]。
#[async_trait::async_trait]
pub trait DepartmentClosureQuery: Send + Sync {
    async fn descendant_ids(
        &self,
        tx: &mut dyn Tx,
        legal_entity_id: Id<LegalEntity>,
        department_id: Id<Department>,
        max_depth: u8,
    ) -> Result<Vec<Id<Department>>, AppError>;
}

/// 一条闭包行：祖先到后代的有向连接，`depth` 为两者间跳数，自零起。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ClosureRow {
    pub ancestor: Id<Department>,
    pub descendant: Id<Department>,
    pub depth: u8,
}

/// 子树全量重写计划。装配侧在同一事务内按序执行三段：
/// 先删、再逐层插、最后维护 `departments.level_no`。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubtreeRewritePlan {
    /// 删除谓词：既有行中 `ancestor_department_id` 落在本集合的全部删除。
    pub delete_ancestors: Vec<Id<Department>>,
    /// 追加删除谓词：改父时旧链残留的跨层行（`descendant_department_id`
    /// 在子树内而祖先在子树外）。新增与停用为空集。
    pub delete_stale_cross_links_for: Vec<Id<Department>>,
    /// 逐层插入的闭包行；每个节点自身行（depth 0）在前，其后按 depth 升序。
    pub inserts: Vec<ClosureRow>,
    /// `departments.level_no` 维护行：部门标识与新的层级序号（自 1 起）。
    pub level_nos: Vec<(Id<Department>, u16)>,
}

/// 组织深度硬上限：防御环状父引用导致的无限展开。
/// 超出即视为数据损坏，返回 None 由调用方映射内部错误。
pub const MAX_ORG_DEPTH: usize = 32;

/// 按子树形态算出全量重写计划。
///
/// - `subtree_layers`：第 0 层恰为子树根，其后各层为子节点集合，层内顺序无关；
/// - `parent_edges`：子树内的父子边（子 → 父），子树根不出现在其中；
/// - `ancestor_chain`：子树根之上的直系祖先链，最近者在前（改父后取新链）；
/// - `stale_ancestor_chain`：改父前的旧祖先链；新增与停用传空。
///
/// 行插入顺序即「逐层插入，depth 自零起」的可执行定义：按层推进，
/// 每个节点先插自身行（depth 0），再按 depth 升序插子树内直系祖先行，
/// 最后是子树外祖先链上的行。祖先一律按 `parent_edges` 走线得出，
/// 同层与旁支节点不会混入。
pub fn plan_subtree_rewrite(
    subtree_layers: &[Vec<Id<Department>>],
    parent_edges: &[(Id<Department>, Id<Department>)],
    ancestor_chain: &[Id<Department>],
    stale_ancestor_chain: &[Id<Department>],
) -> Option<SubtreeRewritePlan> {
    let total_depth = ancestor_chain.len() + subtree_layers.len();
    if total_depth > MAX_ORG_DEPTH || subtree_layers.is_empty() || subtree_layers[0].len() != 1 {
        return None;
    }

    // 每个子树节点相对子树根的层号（根为 0）。
    let mut offset: Vec<(Id<Department>, usize)> = Vec::new();
    for (layer_no, layer) in subtree_layers.iter().enumerate() {
        for id in layer {
            offset.push((*id, layer_no));
        }
    }

    let delete_ancestors: Vec<Id<Department>> = offset.iter().map(|(id, _)| *id).collect();

    let mut inserts: Vec<ClosureRow> = Vec::new();
    for &(node, _node_layer) in &offset {
        let seg_start = inserts.len();
        // 自身行，depth 0。
        inserts.push(ClosureRow {
            ancestor: node,
            descendant: node,
            depth: 0,
        });
        // 子树内直系祖先链：沿 parent_edges 逐跳上行，depth 为跳数。
        let mut cur = node;
        let mut depth: u8 = 1;
        while let Some(&(_, parent)) = parent_edges.iter().find(|&&(child, _)| child == cur) {
            inserts.push(ClosureRow {
                ancestor: parent,
                descendant: node,
                depth,
            });
            cur = parent;
            depth += 1;
            if depth as usize > MAX_ORG_DEPTH {
                return None; // 环状父引用防御
            }
        }
        // 子树外祖先链：depth 续接子树内深度。
        for (i, anc) in ancestor_chain.iter().enumerate() {
            inserts.push(ClosureRow {
                ancestor: *anc,
                descendant: node,
                depth: depth + i as u8,
            });
        }
        // 逐节点内按 depth 升序，落实「depth 自零起」的插入顺序。
        inserts[seg_start..].sort_by_key(|r| r.depth);
    }

    let level_nos: Vec<(Id<Department>, u16)> = offset
        .iter()
        .map(|&(id, layer)| (id, (ancestor_chain.len() + layer + 1) as u16))
        .collect();

    Some(SubtreeRewritePlan {
        delete_ancestors,
        delete_stale_cross_links_for: stale_ancestor_chain.to_vec(),
        inserts,
        level_nos,
    })
}

/// `max_depth` 截止语义：取 0 时只保留本部门（自身行），
/// 取值超过实际深度时按实际深度截止（不产生新行，天然成立）。
pub fn cap_by_depth(rows: Vec<ClosureRow>, max_depth: u8) -> Vec<ClosureRow> {
    rows.into_iter().filter(|r| r.depth <= max_depth).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dept(n: u8) -> Id<Department> {
        Id::<Department>::from_uuid(uuid::Uuid::from_u128(n as u128))
    }

    /// 装配侧以 `Arc<dyn _>` 注入，trait 必须对象安全。
    #[test]
    fn trait_is_object_safe() {
        fn _q(_x: std::sync::Arc<dyn DepartmentClosureQuery>) {}
    }

    #[test]
    fn single_node_subtree_has_only_self_row() {
        let root = dept(1);
        let plan = plan_subtree_rewrite(&[vec![root]], &[], &[], &[]).expect("单子树可算");
        assert_eq!(plan.delete_ancestors, vec![root]);
        assert_eq!(
            plan.inserts,
            vec![ClosureRow {
                ancestor: root,
                descendant: root,
                depth: 0
            }],
            "单节点子树只有自身行"
        );
        assert_eq!(plan.level_nos, vec![(root, 1)], "无上级时根为第 1 层");
    }

    #[test]
    fn rewrite_rows_start_from_depth_zero_per_node() {
        // 根 r，子 a、b，孙 c（a 之下）。
        let (r, a, b, c) = (dept(1), dept(2), dept(3), dept(4));
        let edges = vec![(a, r), (b, r), (c, a)];
        let plan = plan_subtree_rewrite(&[vec![r], vec![a, b], vec![c]], &edges, &[], &[])
            .expect("三层子树可算");
        assert_eq!(plan.delete_ancestors.len(), 4, "子树四节点的既有行全删");
        // b 与 a 同层但互不为祖先：b 不得出现以 a 为祖先的行。
        let b_rows: Vec<&ClosureRow> = plan
            .inserts
            .iter()
            .filter(|row| row.descendant == b)
            .collect();
        assert_eq!(b_rows.len(), 2, "b 只有自身行与父 r 一行");
        assert!(b_rows.iter().all(|row| row.ancestor != a), "旁支不得混入");
        // c 的插入段：自身行在最前且 depth 0。
        let c_rows: Vec<&ClosureRow> = plan
            .inserts
            .iter()
            .filter(|row| row.descendant == c)
            .collect();
        assert_eq!(c_rows.len(), 3, "c 有自身、父 a、祖 r 三行");
        assert_eq!(c_rows[0].depth, 0);
        assert_eq!(c_rows[1].depth, 1);
        assert_eq!(c_rows[2].depth, 2);
        assert_eq!(c_rows[0].ancestor, c);
        assert_eq!(c_rows[1].ancestor, a);
        assert_eq!(c_rows[2].ancestor, r);
        // level_no 同事务维护：c 在第 3 层。
        assert!(plan.level_nos.contains(&(c, 3)));
    }

    #[test]
    fn reparent_extends_chain_and_marks_stale_links() {
        let (r, x, old_p, new_p) = (dept(1), dept(2), dept(3), dept(4));
        let plan =
            plan_subtree_rewrite(&[vec![x]], &[], &[new_p, r], &[old_p]).expect("改父计划可算");
        assert_eq!(
            plan.delete_stale_cross_links_for,
            vec![old_p],
            "旧链残留跨层行要删"
        );
        let depths: Vec<u8> = plan
            .inserts
            .iter()
            .filter(|row| row.descendant == x)
            .map(|row| row.depth)
            .collect();
        assert_eq!(depths, vec![0, 1, 2], "新链两祖先续接 depth");
        assert_eq!(plan.level_nos, vec![(x, 3)], "新链下 x 落第 3 层");
    }

    #[test]
    fn cap_by_depth_zero_returns_self_only() {
        let (a, b) = (dept(1), dept(2));
        let rows = vec![
            ClosureRow {
                ancestor: a,
                descendant: a,
                depth: 0,
            },
            ClosureRow {
                ancestor: a,
                descendant: b,
                depth: 1,
            },
        ];
        let capped = cap_by_depth(rows, 0);
        assert_eq!(capped.len(), 1, "max_depth=0 只返本部门");
        assert_eq!(capped[0].descendant, a);
    }

    #[test]
    fn depth_beyond_org_limit_is_rejected() {
        let chain: Vec<Vec<Id<Department>>> = (0..MAX_ORG_DEPTH + 1)
            .map(|i| vec![dept(i as u8)])
            .collect();
        assert!(
            plan_subtree_rewrite(&chain, &[], &[], &[]).is_none(),
            "超出组织深度上限必须拒绝，防御环状父引用"
        );
    }

    #[test]
    fn layer_zero_must_be_single_root() {
        assert!(plan_subtree_rewrite(&[vec![dept(1), dept(2)]], &[], &[], &[]).is_none());
        assert!(plan_subtree_rewrite(&[], &[], &[], &[]).is_none());
    }

    #[test]
    fn cyclic_parent_edge_is_rejected() {
        // a、b 互为父：沿边走线必然超限，计划必须拒绝。
        let (a, b) = (dept(1), dept(2));
        let layers = vec![vec![a], vec![b]];
        let edges = vec![(b, a), (a, b)];
        // 构造足够深的假象：层数本身不超限，但边走线会打转。
        assert!(plan_subtree_rewrite(&layers, &edges, &[], &[]).is_none());
    }
}
