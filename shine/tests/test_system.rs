use shine::{System, Component, ComponentLifecycle, Injected, async_trait, ComponentMeta, component_registry};
use std::any::TypeId;
use std::sync::Mutex;

#[derive(Component)]
struct A {
    #[injected]
    b: Injected<B>,

    #[injected]
    c: Injected<C>,

    value: Option<i32>
}


#[derive(Component)]
#[lifecycle]
struct B {
    #[injected]
    c: Injected<C>,

    value: Option<i32>
}

#[async_trait]
impl ComponentLifecycle for B {
    async fn start (&mut self) {
        let c_value = self.c.value.lock().unwrap();
        println!("Starting b..., with c_value = {}", c_value);
        self.value = Some(*c_value + 1);
    }
}


#[derive(Component)]
#[lifecycle]
struct C {
    value: Mutex<i32>
}

#[async_trait]
impl ComponentLifecycle for C {
    async fn start (&mut self) {
        println!("Starting c...");
        self.value = Mutex::new(1);
    }
}

component_registry!(SystemRegistry, [
    A, B, C
]);

#[tokio::test]
async fn system_integration_basic() {
    let mut system: System<SystemRegistry> = System::new(
        TypeId::of::<Injected<A>>()
    );

    system.start().await;
    system.stop().await;
}
