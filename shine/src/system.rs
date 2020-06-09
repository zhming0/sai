use std::any::TypeId;
use std::collections::HashSet;
use super::{ComponentMeta, Component};

type GenericComponentMeta = ComponentMeta<dyn Component>;
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
    component_repository: super::ComponentRepository,

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

            component_repository: super::ComponentRepository::new(),
            state: SystemState::Stopped
        }
    }

    async fn start(&mut self) {
        // 1. topology sort
        // 2. start the component one by one
        // 3. Insert started component into repo

    }

    async fn stop(&mut self) {
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
                }
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[derive(Component)]
    //struct A {}
    //#[derive(Component)]
    //struct B {}
    //#[derive(Component)]
    //struct C {}

    //#[test]
    //fn test_topological_sort() {
    //    let registry: ComponentRegistry = |tid| {
    //        if tid == TypeId::of::<A>() {
    //            let m = ComponentMeta {
    //                type_id: TypeId::of::<A>(),
    //                build: |_| A{},
    //                depends_on: vec![
    //                    TypeId::of::<B>(),
    //                    TypeId::of::<C>(),
    //                ]
    //            };
    //            return Some(m)
    //        } else if tid == TypeId::of::<B>() {
    //            let m = ComponentMeta {
    //                type_id: TypeId::of::<B>(),
    //                build: |_| B{},
    //                depends_on: vec![
    //                    TypeId::of::<C>(),
    //                ]
    //            };
    //            return Some(m)
    //        } else {
    //            None
    //        }
    //    };
    //}
}

