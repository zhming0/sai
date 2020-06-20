#[macro_export]
macro_rules! component_registry {
    ($name:ident, [$($x:ty),*]) => {

        pub fn $name (tid: std::any::TypeId) -> Option<$crate::ComponentMeta<Box<dyn $crate::Component>>> {

            $(
                let meta = <$x>::meta();
                if tid == meta.type_id {
                    return Some(meta.into())
                }
            )*

            None
        }
    }
}

#[macro_export]
macro_rules! combine_registries {
    ($name:ident, [$($x:expr),*]) => {

        pub fn $name (tid: std::any::TypeId) -> Option<$crate::ComponentMeta<Box<dyn $crate::Component>>> {

            $(
                let meta = $x(tid);
                if meta.is_some() {
                    return meta;
                }
            )*

            None
        }
    }
}


#[cfg(test)]
mod tests {

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

    component_registry!(dummy_registry, [A]);

    #[test]
    fn component_registry_macro() {
        assert!(matches!(dummy_registry(TypeId::of::<i32>()), None));
        assert!(matches!(dummy_registry(TypeId::of::<Injected<A>>()), Some(_)));
    }

    component_registry!(dummy_registry2, [A]);
    combine_registries!(combined_reg, [dummy_registry, dummy_registry2]);
    #[test]
    fn combine_registries_macro() {
        assert!(matches!(dummy_registry(TypeId::of::<i32>()), None));
        assert!(matches!(dummy_registry(TypeId::of::<Injected<A>>()), Some(_)));
    }
}

