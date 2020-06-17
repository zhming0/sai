use shine::{Component, Injected};
use std::sync::Arc;
use std::any::TypeId;

/*
 * Some testing component
 */
#[derive(Component)]
struct Foo {
    #[injected]
    a: Arc<String>,

    b: String
}


#[test]
fn test_build() {

    let mut repo = shine::ComponentRepository::new();
    let s = String::from("hello world");
    repo.insert(Arc::new(s));

    let foo = Foo::build(&repo);

    assert_eq!(foo.a.as_str(), "hello world");
    assert_eq!(foo.b, "");
}

#[test]
fn test_meta() {

    let meta = Foo::meta();

    assert_eq!(meta.type_id, TypeId::of::<Injected<Foo>>());
    assert_eq!(meta.depends_on, vec![TypeId::of::<Arc<String>>()]);
}
