/*
 * It's arguable if we need this class at all given that we could just use Mutex
 */
use std::sync::{Arc, Mutex};
use std::ops::Deref;
use std::boxed::Box;
use std::any::Any;
use super::downcast::Downcast;

#[derive(Default)]
pub struct Injected<T: ?Sized> {
    item: Option<Arc<T>>,
    item_mut: Option<Arc<Mutex<T>>>
}

impl<T> Injected<T> {
    pub fn new(val: T, mutable: bool) -> Injected<T> {
        if mutable {
            // This is experimental!
            // I don't think this is a good idea
            return Injected {
                item_mut: Some(Arc::new(Mutex::new(val))),
                item: None
            }
        } else {
            return Injected {
                item: Some(Arc::new(val)),
                item_mut: None
            }
        }
    }
}

impl<T: Downcast + 'static + ?Sized> Injected<T> {
    pub fn downcast<S: Any + Send + Sync>(self) -> Option<Injected<S>> {
        if self.item.is_some() {
            let item = self.item.unwrap();
            return Some(Injected {
                item: Some(item.into_any_arc().downcast().unwrap()),
                item_mut: None
            })
        } else {
            return None
        }
    }
}

impl<T: ?Sized> Injected<T> {

    fn from_arc(val: Arc<T>) -> Injected<T> {
        return Injected {
            item: Some(val),
            item_mut: None
        }
    }

    pub fn extract(&self) -> &T {
        if self.item_mut.is_some() {
            panic!("Extract mutable Injected has been implemented!");
        } else {

            let ret = match &self.item {
                Some(v) => v.deref(),
                _ => panic!("Unexpected fatal error in Injected")
            };

            return ret
        }
    }
}

impl<T: ?Sized> Clone for Injected<T> {
    fn clone(&self) -> Injected<T> {
        return Injected {
            item: self.item.clone(),
            item_mut: self.item_mut.clone(),
        }
    }
}

impl<T: ?Sized> From<Box<T>> for Injected<T> {
    fn from(m: Box<T>) -> Self {
        let arc: Arc<T> = m.into();
        return Injected::from_arc(arc);
    }
}
