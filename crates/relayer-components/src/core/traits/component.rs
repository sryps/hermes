use crate::core::traits::sync::Async;

pub trait HasComponents: Async {
    type Components: Async;
}
pub trait DelegateComponent<Name>: Async {
    type Delegate;
}
