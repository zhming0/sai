use shine::{Component, Injected, async_trait};
use gotham::state::State;

#[derive(Component)]
pub struct FooController {

    #[injected]
    db: Injected<super::Db>
}

impl FooController {
    pub fn index (&self) -> Result<String, tide::Error> {
        //std::thread::sleep(std::time::Duration::from_secs(10));
        //println!("---handle index---");
        Ok("Hello Foo".to_string())
    }
}
