use std::any::{Any, TypeId};
use std::boxed::Box;
use std::collections::HashMap;

#[derive(Default)]
pub struct ComponentRepository  {

    repository: HashMap<TypeId, Box<dyn Any>>
}

impl ComponentRepository {

    pub fn new() -> Self {
        ComponentRepository {
            repository: HashMap::new()
        }
    }

    pub fn insert<T: 'static>(&mut self, v: T) {
        self.repository.insert(TypeId::of::<T>(), Box::new(v));
    }

    pub fn insert_with_typeid<T: 'static>(&mut self, type_id: TypeId, v: T) {
        println!("Inserting {:?}", type_id);
        self.repository.insert(type_id, Box::new(v));
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        println!("Getting {:?}", TypeId::of::<T>());
        self.repository
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    pub fn get_by_typeid<T: 'static>(&self, type_id: TypeId) -> Option<&T> {
        println!("Getting {:?}", type_id);
        self.repository
            .get(&type_id)
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Injected;
    use super::super::downcast::Downcast;

    trait T: Downcast { }
    struct A {
        value: u32
    }
    impl T for A {}

    #[test]
    fn test_component_repository() {
        let mut repo = ComponentRepository::new();
        let a = A { value: 0 };
        let boxt:Box<dyn T> = Box::new(a);
        let injected = Injected::from(boxt);

        // assert!(matches!(injected.downcast::<A>(), Some(_)));

        let type_id = TypeId::of::<Injected<A>>();
        repo.insert_with_typeid(type_id, injected);

        let dyn_trait_in_repo: Option<&Injected<dyn T>> = repo.get_by_typeid(type_id);

        assert!(matches!(dyn_trait_in_repo, Some(_)));

        let dyn_trait_clone = dyn_trait_in_repo.unwrap().clone();

        let a_in_repo = dyn_trait_clone.downcast::<A>();
        assert_eq!(a_in_repo.is_some(), true);
        assert_eq!(a_in_repo.unwrap().extract().value, 0);
    }


}
