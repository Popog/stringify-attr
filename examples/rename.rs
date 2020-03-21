use stringify_attr::stringify_braces as stringify_all;

fn main() {
    assert_eq!({
        #[stringify_all{foo}] struct Foo;
        result!()
    }, "#[stringify_all{foo}] struct Foo;");
}
