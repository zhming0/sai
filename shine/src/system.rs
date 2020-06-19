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

        for tid in sorted_type_ids {
            let meta = (self.component_registry)(tid);
            match meta {
                Some(m) => {
                    let type_id = m.type_id;
                    // 2. start the component one by one
                    let mut component = (m.build)(&self.component_repository);
                    component.start().await;

                    // 3. Insert started component into repo
                    let injected_component = Injected::from(component);
                    // Here we need a concrete type so this won't work
                    // self.component_repository.insert(injected_component);
                    // Current solution:
                    self.component_repository.insert_with_typeid(type_id, injected_component);

                },
                None => panic!("This won't happen")
            }
        }


        self.state = SystemState::Started
    }

    pub async fn stop(&mut self) {
        match self.state {
            SystemState::Stopped => return,
            _ => {},
        };
        // 1. topology sort
        let sorted_type_ids = self.topological_sort();

        // In the reversed order of the start
        for tid in sorted_type_ids.into_iter().rev() {
            let component: &mut Injected<dyn Component> = self.component_repository.get_by_typeid_mut(tid).unwrap();
            let owned_component = component.get_mut().unwrap();
            // This is a bit dangerous
            // TODO more documentation
            owned_component.stop().await;
        }


        self.component_repository = ComponentRepository::new();
        self.state = SystemState::Stopped

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
    use async_trait::async_trait;
    use super::super::ComponentLifecycle;

    struct A { }
    impl Component for A {
        fn build(_: &ComponentRepository) -> A { A{} }
        #[inline]
        fn meta() -> ComponentMeta<Box<A>> {
            ComponentMeta {
                type_id: TypeId::of::<Injected<A>>(),
                build: Box::new(|_| Box::new(A{})),
                depends_on: vec![
                    TypeId::of::<Injected<B>>(),
                    TypeId::of::<Injected<C>>(),
                ]
            }
        }
    }
    impl ComponentLifecycle for A {}
    struct B {}
    impl Component for B {
        fn build(_: &ComponentRepository) -> B { B{} }
        fn meta() -> ComponentMeta<Box<B>> {
            ComponentMeta {
                type_id: TypeId::of::<Injected<B>>(),
                build: Box::new(|_| Box::new(B{})),
                depends_on: vec![
                    TypeId::of::<Injected<C>>(),
                ]
            }
        }
    }
    impl ComponentLifecycle for B {}
    struct C {
        number: Option<u32>
    }
    impl Component for C {
        fn build(_: &ComponentRepository) -> C { C{
            number: None
        } }
        fn meta() -> ComponentMeta<Box<C>> {
            ComponentMeta {
                type_id: TypeId::of::<Injected<C>>(),
                build: Box::new(|r: &ComponentRepository| Box::new(C::build(r))),
                depends_on: vec![ ]
            }
        }
    }

    #[async_trait]
    impl ComponentLifecycle for C {
        async fn start(&mut self) {
            self.number = Some(0)
        }
        async fn stop(&mut self) {
            println!("STOPPING C.......");
            self.number = None
        }
    }

    fn demo_registry (tid: TypeId) -> Option<GenericComponentMeta> {
        if tid == TypeId::of::<Injected<A>>() {
            return Some(A::meta().into())
        } else if tid == TypeId::of::<Injected<B>>() {
            return Some(B::meta().into())
        } else if tid == TypeId::of::<Injected<C>>() {
            return Some(C::meta().into())
        } else {
            None
        }
    }


    #[test]
    fn test_topological_sort() {

        let sys = System::new(demo_registry, TypeId::of::<Injected<A>>());
        let result = sys.topological_sort();
        assert_eq!(
            result,
            vec![
                TypeId::of::<Injected<C>>(),
                TypeId::of::<Injected<B>>(),
                TypeId::of::<Injected<A>>(),
            ]
        );
    }

    #[tokio::test]
    async fn test_system_start_stop() {

        let mut system = System::new(
            demo_registry,
            TypeId::of::<Injected<A>>()
        );

        system.start().await;

        let repo = &system.component_repository;
        let type_id = TypeId::of::<Injected<C>>();

        let x: &Injected<dyn Component> = repo.get_by_typeid(type_id).unwrap();
        // TODO need to use week pointer here
        // let c = x.clone().downcast::<C>();
        // assert!(matches!(c.clone().unwrap().extract().number, Some(0)));

        system.stop().await;

        //let c = prec.downcast::<C>();
        // assert!(matches!(c.unwrap().extract().number, None));
    }

    //use std::sync::Arc;
    //#[test]
    //fn experiment () {
    //    let mut x = Some(Arc::new(0));
    //    let y = x.clone();
    //}
}

