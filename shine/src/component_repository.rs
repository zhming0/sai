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

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.repository
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }
}

