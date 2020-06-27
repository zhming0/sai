use std::any::TypeId;
use super::{ Component, ComponentMeta };

/// A macro that helps setting up Component Registry
///
/// ```
/// # use sai::{ComponentMeta, ComponentRepository, Component, ComponentLifecycle, component_registry, Injected};
/// # use std::any::TypeId;
/// # struct A { }
/// # impl Component for A {
/// #     fn build(_: &ComponentRepository) -> A { A{} }
/// #     #[inline]
/// #     fn meta() -> ComponentMeta<Box<A>> {
/// #         ComponentMeta {
/// #             type_id: TypeId::of::<Injected<A>>(),
/// #             build: Box::new(|_| Box::new(A{})),
/// #             depends_on: vec![ ]
/// #         }
/// #     }
/// # }
/// # impl ComponentLifecycle for A {}
/// component_registry!(DummyRegistry, [A]);
/// ```
#[macro_export]
macro_rules! component_registry {
    ($name:ident, [$($x:ty),*]) => {

        struct $name {}

        impl $crate::ComponentRegistry for $name {
            fn get (tid: std::any::TypeId) -> Option<$crate::ComponentMeta<Box<dyn $crate::Component>>> {
                $(
                    let meta = <$x>::meta();
                    if tid == meta.type_id {
                        return Some(meta.into())
                    }
                )*

                None

            }

            fn all () -> Vec<std::any::TypeId> {
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
///
/// ```
/// # use sai::{ComponentMeta, ComponentRepository, Component, ComponentLifecycle, component_registry, Injected, combine_component_registry};
/// # use std::any::TypeId;
/// # struct A { }
/// # impl Component for A {
/// #     fn build(_: &ComponentRepository) -> A { A{} }
/// #     #[inline]
/// #     fn meta() -> ComponentMeta<Box<A>> {
/// #         ComponentMeta {
/// #             type_id: TypeId::of::<Injected<A>>(),
/// #             build: Box::new(|_| Box::new(A{})),
/// #             depends_on: vec![ ]
/// #         }
/// #     }
/// # }
/// # impl ComponentLifecycle for A {}
/// component_registry!(DummyRegistry, [A]);
/// component_registry!(DummyRegistry2, [A]);
///
/// // Combine DummyRegistry and DummyRegistry2 into SuperRegistry
/// combine_component_registry!(SuperRegistry, [ DummyRegistry, DummyRegistry2 ]);
/// ```
#[macro_export]
macro_rules! combine_component_registry {
    ($name:ident, [$($x:ty),*]) => {

        struct $name {}

        impl $crate::ComponentRegistry for $name {
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


/// ComponentRegistry is a **data structure** for system to find a meta information for component.
/// It's required for a system to have a ComponentRegistry.
///
/// Normaly, you don't need to manually implement this trait.
///
/// To define a component registry, you just need to specify the name and a list of Component
/// identifiers.
/// ```
/// use sai::{Component};
/// # use sai::{component_registry};
///
/// #[derive(Component)]
/// struct A {};
/// #[derive(Component)]
/// struct B {};
///
/// component_registry!(ExampleRegistry, [
///     A, B
/// ]);
/// ```
/// Note that A, B above are not values, they are the identifiers.
///
/// In big project, uou can also composite multiple component registires into one.
/// Check out [here](macro.combine_component_registry.html).
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

