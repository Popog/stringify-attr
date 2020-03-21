use stringify_attr::*;

fn main() {
    assert_eq!({
        #[stringify_attr] struct Foo;
        result!()
    }, "");
    assert_eq!({
        #[stringify_attr(foo)] struct Foo;
        result!()
    }, "foo");
    assert_eq!({
        #[stringify_item(foo)] struct Foo;
        result!()
    }, "struct Foo;");
    assert_eq!({
        #[stringify_all(foo)] struct Foo;
        result!()
    }, "#[stringify_all(foo)] struct Foo;");
}
