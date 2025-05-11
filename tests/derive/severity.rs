use error_snippet_derive::Diagnostic;
use insta::assert_snapshot;

use crate::render;

#[test]
fn error() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "some error", severity = Error)]
    struct Foo {}

    assert_snapshot!(render(Foo {}));
}

#[test]
fn warning() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "some warning", severity = Warning)]
    struct Foo {}

    assert_snapshot!(render(Foo {}));
}

#[test]
fn note() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "some note", severity = Note)]
    struct Foo {}

    assert_snapshot!(render(Foo {}));
}

#[test]
fn info() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "some info", severity = Info)]
    struct Foo {}

    assert_snapshot!(render(Foo {}));
}

#[test]
fn help() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "some help", severity = Help)]
    struct Foo {}

    assert_snapshot!(render(Foo {}));
}
