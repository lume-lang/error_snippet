use error_snippet_derive::Diagnostic;
use insta::assert_snapshot;

use crate::render;

#[test]
fn single_line() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "foo", help = "better luck next time!")]
    struct Foo {}

    assert_snapshot!(render(Foo {}));
}

#[test]
fn multiple_lines() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "foo", help = "better luck next time!\nyou'll get there!")]
    struct Foo {}

    assert_snapshot!(render(Foo {}));
}

#[test]
fn multiple_helps() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(
        message = "foo",
        help = "better luck next time!",
        help = "you'll get there!"
    )]
    struct Foo {}

    assert_snapshot!(render(Foo {}));
}
