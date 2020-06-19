use shine::{System, Component, ComponentLifecycle, Injected, async_trait, ComponentMeta};
use std::any::TypeId;

#[derive(Component)]
struct A {
    #[injected]
    b: Injected<B>,

    value: Option<i32>
}


#[derive(Component)]
#[lifecycle]
struct B {
    value: Option<i32>
}

#[async_trait]
impl ComponentLifecycle for B {
    async fn start (&mut self) {
        self.value = Some(0);
    }
}

fn system_registry (tid: TypeId) -> Option<ComponentMeta<Box<dyn Component>>> {
    if tid == TypeId::of::<Injected<A>>() {
        return Some(A::meta().into())
    } else if tid == TypeId::of::<Injected<B>>() {
        return Some(B::meta().into())
    } else {
        None
    }

}

#[tokio::test]
async fn system_integration_1() {
    let mut system = System::new(
        system_registry,
        TypeId::of::<Injected<A>>()
    );

    system.start().await;
    system.stop().await;
}
