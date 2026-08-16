//! 取号的数据库边界。
//!
//! 本 crate 不持有连接、不开事务：取号必须在**调用方的业务事务内**执行，
//! 回滚即退号（基线第 11.1 节）。把数据库操作放在这个 trait 之后，
//! 是为了让上面那条约束在类型上成立——本 crate 拿不到连接，就不可能自己开事务。

use crate::number::{DocumentNumber, LegalEntityCode, PeriodKey, SequenceError, TypeCode};
use crate::registry::{ScopeKind, TypeCodeRegistry};

/// 一次取号的入参。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AllocateRequest {
    pub scope_kind: ScopeKind,
    pub type_code: TypeCode,
    pub legal_entity_code: LegalEntityCode,
    pub period_key: PeriodKey,
}

/// 取号语句的返回值，与 SQL 的 `returning next_value as serial_value,
/// width as effective_width` 两列一一对应。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Allocated {
    pub serial: u64,
    pub effective_width: u8,
}

/// 由 `ep-adapter-db-pg` 实现。实现体就是计划第 3.4.1 节那条
/// `update ... returning` 语句，本 crate 不复述 SQL。
pub trait NumberAllocator {
    fn allocate(&mut self, req: &AllocateRequest) -> Result<Allocated, SequenceError>;
}

/// 取号算法的第 1 步与第 5 步：校验类型码已登记，取号，按返回的位数格式化。
///
/// 中间三步（算期间键、保证序列行存在、执行取号语句）分别落在调用方与
/// [`NumberAllocator`] 的实现体内——期间键由调用方按记账日期或业务日期给出，
/// 因为只有它知道该用哪个日期；后两步是同一条事务内的数据库操作。
pub fn allocate(
    registry: &TypeCodeRegistry,
    allocator: &mut dyn NumberAllocator,
    req: &AllocateRequest,
) -> Result<DocumentNumber, SequenceError> {
    registry.ensure_registered(&req.type_code)?;
    let got = allocator.allocate(req)?;
    Ok(DocumentNumber {
        type_code: req.type_code.clone(),
        legal_entity_code: req.legal_entity_code.clone(),
        period_key: req.period_key.clone(),
        serial: got.serial,
        width: got.effective_width,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::number::next_width;

    fn tc(chars: &[char]) -> TypeCode {
        TypeCode::from_chars(chars.iter().copied()).expect("夹具类型码应合法")
    }

    fn req(code: TypeCode) -> AllocateRequest {
        AllocateRequest {
            scope_kind: ScopeKind::Document,
            type_code: code,
            legal_entity_code: LegalEntityCode::parse("01").expect("法人码"),
            period_key: PeriodKey::parse("202608").expect("期间键"),
        }
    }

    /// 内存版取号器，语义与那条 SQL 逐句对齐：先按旧值判位数，再自增。
    struct MemAllocator {
        next_value: u64,
        width: u8,
        calls: u32,
    }

    impl NumberAllocator for MemAllocator {
        fn allocate(&mut self, _req: &AllocateRequest) -> Result<Allocated, SequenceError> {
            self.calls += 1;
            let effective_width = next_width(self.next_value, self.width);
            self.next_value += 1;
            self.width = effective_width;
            Ok(Allocated {
                serial: self.next_value,
                effective_width,
            })
        }
    }

    #[test]
    fn unregistered_type_code_never_reaches_the_allocator() {
        let registry = TypeCodeRegistry::from_registered(); // 空表
        let mut alloc = MemAllocator {
            next_value: 0,
            width: 6,
            calls: 0,
        };
        let err = allocate(&registry, &mut alloc, &req(tc(&['S', 'O']))).expect_err("空表必须拒绝");
        assert!(matches!(err, SequenceError::TypeCodeNotRegistered { .. }));
        // 这条是本用例的要点：校验没过就不该动序列。若顺序写反，
        // 一个未登记的类型码会白白吃掉一个号——而回滚退号只在事务内成立，
        // 校验失败时调用方未必回滚。
        assert_eq!(alloc.calls, 0, "校验未过时不得调用取号器");
    }

    #[test]
    fn serial_increments_and_width_expands_across_the_boundary() {
        let code = tc(&['S', 'O']);
        let registry = TypeCodeRegistry::new(vec![code.clone()]);
        let mut alloc = MemAllocator {
            next_value: 999_998,
            width: 6,
            calls: 0,
        };
        let a = allocate(&registry, &mut alloc, &req(code.clone())).expect("第一次");
        let b = allocate(&registry, &mut alloc, &req(code.clone())).expect("第二次");
        let c = allocate(&registry, &mut alloc, &req(code)).expect("第三次");
        assert!(a.to_string().ends_with("-999999"), "实际 {a}");
        assert_eq!(b.width, 7, "跨界这一次就要扩位");
        assert!(b.to_string().ends_with("-1000000"), "实际 {b}");
        assert!(c.to_string().ends_with("-1000001"), "实际 {c}");
    }
}
