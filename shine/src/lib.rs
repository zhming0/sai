use std::sync::Arc;
use std::collections::HashMap;
use std::boxed::Box;
#[macro_use]
extern crate downcast_rs;
use downcast_rs::DowncastSync;

pub trait Component: DowncastSync {
    fn start(&mut self) -> ();
}
impl_downcast!(sync Component);

pub type ComponentRegistry = HashMap<String, Box<dyn Component>>;

/*
 * Some testing component
 */
struct Foo {
}

impl Component for Foo {
    fn start(&mut self) {
    }
}

struct Bar {
}

impl Component for Bar {
    fn start(&mut self) {
    }
}

fn need_foo(f: &Foo) {
    println!("Got Foo");
}
/*
 * --------------------
 */

struct ComponentInfo<T> {
    name: String,
    // For topolofy calculartion
    depends_on: Vec<String>,

    initialize: dyn Fn(&ComponentRegistry) -> Arc<T>
}

// A list of ComponentInfo -> Container
pub fn create_container() {

    let mut registry: ComponentRegistry = HashMap::new();

    let foo = Foo {};
    registry.insert("Foo".to_string(), Box::new(foo));

    let mut bar = Box::new(Bar {});
    bar.start();
    registry.insert("Bar".to_string(), bar);

    let foo_2 = registry.get("Foo").unwrap();

    need_foo(foo_2.downcast_ref::<Foo>().unwrap());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        create_container();
        assert_eq!(2 + 2, 4);
    }
}
