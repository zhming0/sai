use std::any::TypeId;
use std::collections::HashSet;
use super::{Component, ComponentRepository, Injected, ComponentRegistry};

enum SystemState {
    Stopped,
    Started
}

/// **A system is a collection of components** + the ability to control the lifecycle
/// of components in a way meeting the dependency requirement of components, e.g. start/stop them.
///
///
/// ```
///
/// // Assume a RootRegistry is defined here
/// use sai::{System, Component, component_registry};
///
/// #[derive(Component)]
/// struct Foo {
/// }
///
/// component_registry!(RootRegistry, [Foo]);
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut system : System<RootRegistry> = System::new();
///     println!("System starting up...");
///     system.start().await;
///     println!("System started.");
///
///     // Waiting for Ctrl-c
///     // signal::ctrl_c().await?;
///
///     println!("System shutting down...");
///     system.stop().await;
///     println!("System shutted down.");
///     Ok(())
/// }
/// ```
pub struct System<T> where T: ComponentRegistry {

    /// If this is set, then the system will only start
    /// components that can be reached by this entrypoint.
    pub entrypoint: Option<TypeId>,

    /*
     * Just a dummy object to store the type
     * Any better way?
     */
     __dummy: T,

    /*
     Initiated components
     */
    component_repository: ComponentRepository,

    state: SystemState
}

impl<T> System<T> where T: ComponentRegistry {

    /// Create a new system with a Component Registry
    ///
    /// __Example__
    /// ```ignore
    /// use sai::{component_registry, System};
    /// component_registry!(RootRegistry, [ component1, component2 ]);
    ///
    /// let system: System<RootRegistry> = System::new();
    /// ```
    ///
    pub fn new() -> Self {
        return System {
            entrypoint: None,
            __dummy: T::new(),
            component_repository: ComponentRepository::new(),
            state: SystemState::Stopped
        }
    }

    /// Similar to System::new() but allow you to specified an entrypoint for the system.
    /// If an entrypoint is specified, it will become the topology root of component tree.
    pub fn with_entrypoint(
        entrypoint: TypeId
    ) -> Self {
        return System {
            entrypoint: Some(entrypoint),
            __dummy: T::new(),
            component_repository: ComponentRepository::new(),
            state: SystemState::Stopped
        }
    }

    /// Create & start all components in the registry in a topological order.
    /// The topological order is automatically derived by system from analysing `#[injected]`
    /// macro attributes in component definitons.
    ///
    /// The entrypoints will be automatically detected unless specifically specified.
    ///
    /// following the example above:
    /// ```ignore
    /// system.start().await;
    /// ```
    pub async fn start(&mut self) {
        match self.state {
            SystemState::Started => return,
            _ => {},
        };
        // 1. topology sort
        let sorted_type_ids = self.topological_sort();

        for tid in sorted_type_ids {
            let meta = T::get(tid);
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

    /// Stop and **drop** all components in a topological order in reverse to startup.
    /// A typical example used with tokio signal:
    /// ```ignore
    /// use tokio::signal;
    ///
    /// ...
    ///
    /// // Waiting for Ctrl-c
    /// signal::ctrl_c().await?;

    /// println!("System shutting down...");
    /// system.stop().await;
    /// println!("System shutted down.");
    ///
    /// ```
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

            // Force rust to drop memory
            self.component_repository.remove_by_typeid(tid);
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

        let entrypoints = {
            if self.entrypoint.is_some() {
                vec![self.entrypoint.unwrap()]
            } else {
                Self::detect_entrypoints()
            }
        };

        stack.append(&mut entrypoints.clone());
        for e in entrypoints {
            in_stack.insert(e);
        }

        //stack.push(self.entrypoint);
        //in_stack.insert(self.entrypoint);

        while let Some(current_type_id) = stack.last() {
            // TODO: error handling
            let current_meta = T::get(*current_type_id).unwrap();
            let depends_on = &current_meta.depends_on;
            let next_target = depends_on
                .iter()
                .find(|tid| !in_results.contains(tid));
            match next_target {
                Some(t) => {
                    if in_stack.contains(t) {
                        panic!("Unable to handle circular dependency in the system");
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

    fn detect_entrypoints () -> Vec<TypeId> {
        // If a tid has dependecy, it will be flagged here
        let mut flagged = std::collections::HashSet::new();

        let all_tids = T::all();
        for tid in all_tids {
            let meta = T::get(tid).unwrap();
            let depends = meta.depends_on;
            for t in depends {
                flagged.insert(t);
            }
        }

        T::all()
            .into_iter()
            .filter(|id| !flagged.contains(&id))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use super::super::{ComponentLifecycle, ComponentMeta};

    type GenericComponentMeta = ComponentMeta<Box<dyn Component>>;


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

    struct DemoRegistry { }
    impl ComponentRegistry for DemoRegistry {
        fn get (tid: TypeId) -> Option<GenericComponentMeta> {
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
        fn all () -> Vec<TypeId> {
            vec![
                TypeId::of::<Injected<A>>(),
                TypeId::of::<Injected<B>>(),
                TypeId::of::<Injected<C>>(),
            ]
        }

        fn new() -> Self {
            DemoRegistry {}
        }
    }


    #[test]
    fn test_topological_sort() {

        let sys: System<DemoRegistry> = System::new();
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

        let mut system: System<DemoRegistry> = System::new();

        system.start().await;


        // I wish I can assert the `x` was changed in place
        // But I can't because x will have to dropped after stop
        // And x can only be dropped if there is no dependecy on it

        // let repo = &system.component_repository;
        // let type_id = TypeId::of::<Injected<C>>();
        // let x: &Injected<dyn Component> = repo.get_by_typeid(type_id).unwrap();

        // let c = x.clone().downcast::<C>();
        // assert!(matches!(c.clone().unwrap().extract().number, Some(0)));

        system.stop().await;

        //let c = prec.downcast::<C>();
        // assert!(matches!(c.unwrap().extract().number, None));
    }
}

