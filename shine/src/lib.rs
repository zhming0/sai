use std::boxed::Box;
use std::any::TypeId;

pub use component_derive::Component;

mod injected;
pub use injected::Injected;

mod component_repository;
pub use component_repository::ComponentRepository;

mod system;
pub use system::System;

pub trait Component {
    fn start(&mut self) {}
    fn stop(&mut self) {}

    fn build(registry: &ComponentRepository) -> Self
        where Self: Sized;

    fn meta() -> ComponentMeta<Self>
        where Self: Sized;
}

#[derive(Clone)]
pub struct ComponentMeta<T: ?Sized> {
    pub depends_on: Vec<TypeId>,
    pub build: fn(&ComponentRepository) -> T,
    pub type_id: TypeId
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
