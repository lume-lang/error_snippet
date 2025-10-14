use error_snippet_derive::Diagnostic;
use insta::assert_snapshot;

use crate::render;

#[test]
fn only_message() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "some error")]
    struct Foo {}

    assert_snapshot!(render(Foo {}));
}

#[test]
fn formatted_message() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "foo {name}")]
    struct Foo {
        pub name: &'static str,
    }

    assert_snapshot!(render(Foo { name: "bar" }));
}

#[test]
fn formatted_message_debug() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "foo {name:?}")]
    struct Foo {
        pub name: &'static str,
    }

    assert_snapshot!(render(Foo { name: "bar" }));
}

#[test]
fn formatted_message_debug_pretty() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "foo {name:#?}")]
    struct Foo {
        pub name: &'static [&'static str],
    }

    assert_snapshot!(render(Foo { name: &["bar", "baz"] }));
}

#[test]
fn multiple_formatted_message() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "{name1} {name2}")]
    struct Foo {
        pub name1: &'static str,

        pub name2: &'static str,
    }

    assert_snapshot!(render(Foo {
        name1: "foo",
        name2: "bar",
    }));
}
