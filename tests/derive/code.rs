use error_snippet_derive::Diagnostic;
use insta::assert_snapshot;

use crate::render;

#[test]
fn simple_code() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "some error", code = "E3404")]
    struct Foo {}

    assert_snapshot!(render(Foo {}));
}

#[test]
#[allow(dead_code)]
fn unformatted() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "some error", code = "E{code}")]
    struct Foo {
        pub code: usize,
    }

    assert_snapshot!(render(Foo { code: 3404 }));
}

#[test]
#[allow(dead_code)]
fn complex() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "some error", code = "some::obtuse::code")]
    struct Foo {}

    assert_snapshot!(render(Foo {}));
}
