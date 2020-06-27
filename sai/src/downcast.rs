use std::any::Any;
use std::sync::Arc;


pub trait Downcast : Any {
    fn as_any(&self) -> &dyn Any;
    fn into_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
}

impl<T: Any + Send + Sync> Downcast for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn into_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Wrapper<T: ?Sized> {
        value: Arc<T>
    }
    impl<T: Downcast + 'static + ?Sized> Wrapper<T> {
        fn downcast<S: Any + Send + Sync>(self) -> Option<Wrapper<S>> {
            if self.value.as_any().is::<S>() {
                Some(Wrapper {
                    value: self.value.into_any_arc().downcast().unwrap()
                })
            } else {
                None
            }
        }
    }
    trait Trait: Downcast { }
    struct St {}
    impl Trait for St {}

    #[test]
    fn downcast() {
        let arc = Arc::new(St{});
        let x: Wrapper<dyn Trait> = Wrapper {
            value: arc
        };

        assert!(matches!(x.downcast::<St>(), Some(_)));
    }
}
