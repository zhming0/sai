use std::any::TypeId;
use std::collections::HashSet;
use super::{ComponentMeta, Component, ComponentRepository, Injected};

type GenericComponentMeta = ComponentMeta<Box<dyn Component>>;
type ComponentRegistry = fn(TypeId) -> Option<GenericComponentMeta>;

enum SystemState {
    Stopped,
    Started
}

pub struct System {
    pub component_registry: ComponentRegistry,

    pub entrypoint: TypeId,

    /*
     Initiated components
     */
    component_repository: ComponentRepository,

    state: SystemState
}

impl System {

    pub fn new(
        component_registry: ComponentRegistry,
        entrypoint: TypeId
    ) -> Self {
        return System {
            component_registry,
            entrypoint,

            component_repository: ComponentRepository::new(),
            state: SystemState::Stopped
        }
    }

    pub async fn start(&mut self) {
        match self.state {
            SystemState::Started => return,
            _ => {},
        };
        // 1. topology sort
        let sorted_type_ids = self.topological_sort();

        // 2. start the component one by one
        for tid in sorted_type_ids {
            let meta = (self.component_registry)(tid);
            match meta {
                Some(m) => {
                    let component = (m.build)(&self.component_repository);
                    // let a: std::sync::Arc<dyn Component> = component.into();
                    let injected_component: Injected<dyn Component> = component.into();
                    let v = injected_component.extract();
                    // v.start().await;
                    self.component_repository.insert(injected_component);
                    // Do I turn this into an Injected and let it inject?

                },
                None => panic!("This won't happen")
            }
        }
        // 3. Insert started component into repo


        self.state = SystemState::Started
    }

    pub async fn stop(&mut self) {
        // 1. topology sort
        // 2. stop components one by one
        // 3. Remove started component from repo

    }

    fn topological_sort(&self) -> Vec<TypeId> {
        // cycle detection
        let mut in_results: HashSet<TypeId> = HashSet::new();
        let mut result: Vec<TypeId> = Vec::new();
        let mut stack: Vec<TypeId> = Vec::new();
        let mut in_stack: HashSet<TypeId> = HashSet::new();

        stack.push(self.entrypoint);
        in_stack.insert(self.entrypoint);

        while let Some(current_type_id) = stack.last() {
            // TODO: error handling
            let current_meta = (self.component_registry)(*current_type_id).unwrap();
            let depends_on = &current_meta.depends_on;
            let next_target = depends_on
                .iter()
                .find(|tid| !in_results.contains(tid));
            match next_target {
                Some(t) => {
                    if in_stack.contains(t) {
                        panic!("GG");
                    }
                    stack.push(*t);
                    in_stack.insert(*t);
                },
                None => {
                    result.push(*current_type_id);
                    in_results.insert(*current_type_id);
                    in_stack.remove(current_type_id);
                    stack.pop();
                }
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct A { }
    impl Component for A {
        fn build(_: &ComponentRepository) -> A { A{} }
        #[inline]
        fn meta() -> ComponentMeta<Box<A>> {
            ComponentMeta {
                type_id: TypeId::of::<A>(),
                build: Box::new(|_| Box::new(A{})),
                depends_on: vec![
                    TypeId::of::<B>(),
                    TypeId::of::<C>(),
                ]
            }
        }
    }
    struct B {}
    impl Component for B {
        fn build(_: &ComponentRepository) -> B { B{} }
        fn meta() -> ComponentMeta<Box<B>> {
            ComponentMeta {
                type_id: TypeId::of::<B>(),
                build: Box::new(|_| Box::new(B{})),
                depends_on: vec![
                    TypeId::of::<C>(),
                ]
            }
        }
    }
    struct C {}
    impl Component for C {
        fn build(_: &ComponentRepository) -> C { C{} }
        fn meta() -> ComponentMeta<Box<C>> {
            ComponentMeta {
                type_id: TypeId::of::<C>(),
                build: Box::new(|_| Box::new(C{})),
                depends_on: vec![ ]
            }
        }
    }


    #[test]
    fn test_topological_sort() {

        let registry: ComponentRegistry = |tid| {
            if tid == TypeId::of::<A>() {
                return Some(A::meta().into())
            } else if tid == TypeId::of::<B>() {
                return Some(B::meta().into())
            } else if tid == TypeId::of::<C>() {
                return Some(C::meta().into())
            } else {
                None
            }
        };

        let sys = System::new(registry, TypeId::of::<A>());
        let result = sys.topological_sort();
        assert_eq!(
            result,
            vec![
                TypeId::of::<C>(),
                TypeId::of::<B>(),
                TypeId::of::<A>(),
            ]
        );
    }
}

