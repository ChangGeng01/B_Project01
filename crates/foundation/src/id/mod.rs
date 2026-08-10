//! 类型化标识符。

pub mod marker;

use core::marker::PhantomData;

/// 跨模块引用的类型化 UUID。
///
/// `PhantomData<fn() -> T>` 使 `Id<T>` 的 Send/Sync/Copy 不受 `T` 约束，
/// 标记类型因此可以是无任何 trait 实现的零大小类型（第 1.3 节准入判据）。
pub struct Id<T> {
    value: uuid::Uuid,
    _marker: PhantomData<fn() -> T>,
}

impl<T> Id<T> {
    pub const fn from_uuid(value: uuid::Uuid) -> Self {
        Self { value, _marker: PhantomData }
    }

    pub const fn as_uuid(&self) -> uuid::Uuid {
        self.value
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for Id<T> {}

impl<T> core::hash::Hash for Id<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl<T> core::fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Id({})", self.value)
    }
}

impl<T> core::fmt::Display for Id<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.value, f)
    }
}
