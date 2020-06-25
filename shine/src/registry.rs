use std::any::TypeId;
use super::{ Component, ComponentMeta };

/// A macro that helps setting up Component Registry
#[macro_export]
macro_rules! component_registry {
    ($name:ident, [$($x:ty),*]) => {

        struct $name {}

        impl $crate::registry::ComponentRegistry for $name {
            fn get (tid: std::any::TypeId) -> Option<$crate::ComponentMeta<Box<dyn $crate::Component>>> {
                $(
                    let meta = <$x>::meta();
                    if tid == meta.type_id {
                        return Some(meta.into())
                    }
                )*

                None

            }

            fn all () -> Vec<TypeId> {
                vec![
                    $(
                        std::any::TypeId::of::<$crate::Injected<$x>>(),
                    )*
                ]
            }

            fn new () -> Self {
                $name{}
            }
        }

    }
}

/// A macro that combines any number of Component Registry
#[macro_export]
macro_rules! combine_component_registry {
    ($name:ident, [$($x:ty),*]) => {

        struct $name {}

        impl $crate::registry::ComponentRegistry for $name {
            fn get (tid: std::any::TypeId) -> Option<$crate::ComponentMeta<Box<dyn $crate::Component>>> {
                $(
                    let meta = <$x>::get(tid);
                    if meta.is_some() {
                        return meta;
                    }
                )*

                None

            }

            fn all () -> Vec<TypeId> {
                let mut result = Vec::new();
                $(
                    let mut all = <$x>::all();
                    result.append(&mut all);
                )*
                return result;
            }

            fn new () -> Self {
                $name{}
            }
        }

    }
}


pub trait ComponentRegistry {
    /// Getting a
    fn get (type_id: TypeId) -> Option<ComponentMeta<Box<dyn Component>>>;

    /// All the TypeIds that's in this registry
    fn all () -> Vec<TypeId>;

    fn new () -> Self;
}


#[cfg(test)]
mod tests {

    use super::*;
    use super::super::*;
    use std::any::TypeId;

    // A manually implemented component
    struct A { }
    impl Component for A {
        fn build(_: &ComponentRepository) -> A { A{} }
        #[inline]
        fn meta() -> ComponentMeta<Box<A>> {
            ComponentMeta {
                type_id: TypeId::of::<Injected<A>>(),
                build: Box::new(|_| Box::new(A{})),
                depends_on: vec![ ]
            }
        }
    }
    impl ComponentLifecycle for A {}

    component_registry!(DummyRegistry, [A]);
    component_registry!(DummyRegistry2, [A]);
    combine_component_registry!(CombinedRegistry, [DummyRegistry, DummyRegistry2]);

    #[test]
    fn component_registry_new_macro() {
        assert!(matches!(DummyRegistry::get(TypeId::of::<i32>()), None));
        assert!(matches!(DummyRegistry::get(TypeId::of::<Injected<A>>()), Some(_)));

        assert_eq!(DummyRegistry::all(), vec![TypeId::of::<Injected<A>>()]);
    }

    #[test]
    fn combine_registries_new_macro() {
        assert!(matches!(CombinedRegistry::get(TypeId::of::<i32>()), None));
        assert!(matches!(CombinedRegistry::get(TypeId::of::<Injected<A>>()), Some(_)));
    }
}

