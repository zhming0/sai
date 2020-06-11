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

    fn meta() -> ComponentMeta<Box<Self>>
        where Self: Sized;
}

pub struct ComponentMeta<T: ?Sized> {
    pub depends_on: Vec<TypeId>,
    pub type_id: TypeId,
    pub build: Box<dyn Fn(&ComponentRepository) -> T>
}

impl<T: Component + 'static> From<ComponentMeta<Box<T>>> for ComponentMeta<Box<dyn Component>> {

    fn from(m: ComponentMeta<Box<T>>) -> Self {
        ComponentMeta {
            depends_on: m.depends_on.clone(),
            type_id: m.type_id,
            build: Box::new(move |r: &ComponentRepository| (m.build)(r))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
