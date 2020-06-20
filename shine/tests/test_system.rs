use shine::{System, Component, ComponentLifecycle, Injected, async_trait, ComponentMeta, component_registry};
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

component_registry!(system_registry, [
    A, B
]);

#[tokio::test]
async fn system_integration_basic() {
    let mut system = System::new(
        system_registry,
        TypeId::of::<Injected<A>>()
    );

    system.start().await;
    system.stop().await;
}
