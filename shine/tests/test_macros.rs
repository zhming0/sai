use shine::Component;

/*
 * Some testing component
 */
#[derive(Component)]
struct Foo {
    #[injected(name="hello world")]
    a: String
}


#[test]
fn test_what() {

    let repo = shine::ComponentRepository::new();

    let foo = Foo::build(&repo);

    //let mut f = Foo {
    //    //a: "hello world".to_string()
    //};
    //f.start();

    assert_eq!(2 + 2, 4);
}
