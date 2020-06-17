use shine::{System, Component, ComponentLifecycle, Injected, async_trait};

#[derive(Component)]
struct A {
    #[injected]
    b: Injected<String>,

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

#[test]
fn system_integration_1() {
}
