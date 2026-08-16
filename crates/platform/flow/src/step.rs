//! 步骤幂等键与运行约束。
//!
//! 幂等键的形状是崩溃恢复正确性的一半：计划第 3.4.8 节要求
//! 「在步骤执行前、执行中、提交后和补偿过程中随机终止核心进程，恢复后实例状态、
//! 业务效果与审计三者一致」。一步一事务保证了「要么全做要么全不做」，
//! 而幂等键保证了「重放同一步不会做第二遍」——两者缺一不可。

/// 步骤幂等键。形状固定为 `<instance_id>:<node_id>:<execution_no>`
/// （计划第 3.4.8 节逐字）。
///
/// 该键既写入 `process_steps.idempotency_key`（有唯一约束），
/// 也作为 `Idempotency-Key` 传给被调用的用例——**两处必须是同一个值**，
/// 否则重放时步骤表拦住了、被调方却没拦住，副作用照做第二遍。
pub fn idempotency_key(instance_id: &str, node_id: &str, execution_no: u32) -> String {
    format!("{instance_id}:{node_id}:{execution_no}")
}

/// 由该实例该节点的已有步骤数算出本次的 `execution_no`。
///
/// 计划逐字：「`execution_no` 由该实例该节点的已有步骤数决定」。
/// 从 0 起——一个节点第一次执行时已有步骤数为 0。
/// **重试同一节点时该值递增**，因此重试的幂等键与首次不同，
/// 这是有意的：重试是一次新的执行，不该被首次的幂等键挡住。
pub fn execution_no(existing_steps_for_node: u32) -> u32 {
    existing_steps_for_node
}

/// 运行约束三项。取值出处是规格第 9.1 章与计划第 3.4.8 节。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RunLimits {
    pub max_instance_duration_days: u32,
    pub max_steps_per_instance: u32,
    pub max_parallel_branches: u32,
}

/// 当前实例的运行实况，用于比对上限。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RunFacts {
    pub elapsed_days: u32,
    pub steps_so_far: u32,
    pub parallel_branches: u32,
}

/// 触及了哪一项上限。**逐项分开报，不合成一个布尔**：
/// 「超限了」这三个字对运维没有任何帮助，他要知道的是超的哪一项、
/// 当前值多少、上限多少——写 LIMIT_EXCEEDED 人工任务时这三样都要填进去。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LimitBreach {
    Duration { elapsed_days: u32, limit: u32 },
    Steps { steps: u32, limit: u32 },
    ParallelBranches { branches: u32, limit: u32 },
}

impl LimitBreach {
    /// 供人工任务与告警使用的短标识。
    pub fn kind(self) -> &'static str {
        match self {
            LimitBreach::Duration { .. } => "MAX_INSTANCE_DURATION_DAYS",
            LimitBreach::Steps { .. } => "MAX_STEPS_PER_INSTANCE",
            LimitBreach::ParallelBranches { .. } => "MAX_PARALLEL_BRANCHES",
        }
    }
}

/// 检查三项上限。返回第一项触及的，`None` 表示都在限内。
///
/// 检查次序是运行期、步骤数、并行分支数——按「最难恢复的排在前面」排：
/// 一个跑了太久的实例通常是卡住了，先报它比先报分支数更有用。
/// 次序只影响同时超两项时先报哪个，不影响是否拦截。
pub fn check_limits(limits: &RunLimits, facts: &RunFacts) -> Option<LimitBreach> {
    if facts.elapsed_days > limits.max_instance_duration_days {
        return Some(LimitBreach::Duration {
            elapsed_days: facts.elapsed_days,
            limit: limits.max_instance_duration_days,
        });
    }
    if facts.steps_so_far > limits.max_steps_per_instance {
        return Some(LimitBreach::Steps {
            steps: facts.steps_so_far,
            limit: limits.max_steps_per_instance,
        });
    }
    if facts.parallel_branches > limits.max_parallel_branches {
        return Some(LimitBreach::ParallelBranches {
            branches: facts.parallel_branches,
            limit: limits.max_parallel_branches,
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_shape_is_fixed() {
        assert_eq!(idempotency_key("inst-1", "node-a", 0), "inst-1:node-a:0");
        assert_eq!(idempotency_key("inst-1", "node-a", 3), "inst-1:node-a:3");
    }

    /// 同一实例同一节点重试时，键必须变——重试是一次新的执行。
    /// 若键不变，重试会被步骤表的唯一约束挡住，实例就此卡死。
    #[test]
    fn retry_of_the_same_node_gets_a_new_key() {
        let first = idempotency_key("i", "n", execution_no(0));
        let retry = idempotency_key("i", "n", execution_no(1));
        assert_ne!(first, retry);
    }

    /// 不同实例、不同节点之间的键不得相撞。
    #[test]
    fn keys_do_not_collide_across_instances_or_nodes() {
        assert_ne!(idempotency_key("i1", "n", 0), idempotency_key("i2", "n", 0));
        assert_ne!(idempotency_key("i", "n1", 0), idempotency_key("i", "n2", 0));
    }

    fn limits() -> RunLimits {
        RunLimits {
            max_instance_duration_days: 30,
            max_steps_per_instance: 500,
            max_parallel_branches: 16,
        }
    }

    /// 正好等于上限不算超——上限是「不得超过」，不是「不得达到」。
    /// 这个差一是最常写反的地方：写成 `>=` 会让一个刚好用满配额的
    /// 合法实例被打进人工干预。
    #[test]
    fn equal_to_the_limit_is_not_a_breach() {
        let f = RunFacts {
            elapsed_days: 30,
            steps_so_far: 500,
            parallel_branches: 16,
        };
        assert_eq!(check_limits(&limits(), &f), None);
    }

    #[test]
    fn each_limit_is_reported_separately_with_its_numbers() {
        let l = limits();
        let over_days = RunFacts {
            elapsed_days: 31,
            steps_so_far: 1,
            parallel_branches: 1,
        };
        assert_eq!(
            check_limits(&l, &over_days),
            Some(LimitBreach::Duration {
                elapsed_days: 31,
                limit: 30
            })
        );
        let over_steps = RunFacts {
            elapsed_days: 1,
            steps_so_far: 501,
            parallel_branches: 1,
        };
        assert_eq!(
            check_limits(&l, &over_steps).map(LimitBreach::kind),
            Some("MAX_STEPS_PER_INSTANCE")
        );
        let over_branches = RunFacts {
            elapsed_days: 1,
            steps_so_far: 1,
            parallel_branches: 17,
        };
        assert_eq!(
            check_limits(&l, &over_branches).map(LimitBreach::kind),
            Some("MAX_PARALLEL_BRANCHES")
        );
    }

    /// 同时超两项时先报运行期——它通常意味着实例卡住了，比分支数更值得先看。
    #[test]
    fn duration_is_reported_first_when_several_breach() {
        let f = RunFacts {
            elapsed_days: 99,
            steps_so_far: 9999,
            parallel_branches: 99,
        };
        assert_eq!(
            check_limits(&limits(), &f).map(LimitBreach::kind),
            Some("MAX_INSTANCE_DURATION_DAYS")
        );
    }
}
