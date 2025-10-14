use std::ops::Range;
use std::sync::Arc;

use error_snippet::{NamedSource, WithSource};
use error_snippet_derive::Diagnostic;
use insta::assert_snapshot;

use crate::render;

#[test]
fn single_label() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "foo")]
    struct Foo {
        #[span]
        pub source: Arc<NamedSource>,

        #[label("label here")]
        pub span: Range<usize>,
    }

    let source = Arc::new(NamedSource::new(
        "some_file.lm",
        r#"fn main() -> void {
    return 0;
}
"#,
    ));

    assert_snapshot!(render(Foo { source, span: 13..17 }));
}

#[test]
fn multiple_labels() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "foo")]
    struct Foo {
        #[span]
        pub source: Arc<NamedSource>,

        #[label("label 1 here")]
        pub span1: Range<usize>,

        #[label("label 2 here")]
        pub span2: Range<usize>,
    }

    let source = Arc::new(NamedSource::new(
        "some_file.lm",
        r#"fn main() -> void {
    return 0;
}
"#,
    ));

    assert_snapshot!(render(Foo {
        source,
        span1: 13..17,
        span2: 24..30,
    }));
}

#[test]
fn labels_delayed_source() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "foo")]
    struct Foo {
        #[label("label 1 here")]
        pub span1: Range<usize>,

        #[label("label 2 here")]
        pub span2: Range<usize>,
    }

    let source = Arc::new(NamedSource::new(
        "some_file.lm",
        r#"fn main() -> void {
    return 0;
}
"#,
    ));

    assert_snapshot!(render(
        Foo {
            span1: 13..17,
            span2: 24..30,
        }
        .with_source(source)
    ));
}

#[test]
fn label_string_source() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "foo")]
    struct Foo {
        #[span]
        pub source: Arc<String>,

        #[label("label here")]
        pub span: Range<usize>,
    }

    let source = Arc::new(
        r#"fn main() -> void {
    return 0;
}
"#
        .to_string(),
    );

    assert_snapshot!(render(Foo { source, span: 13..17 }));
}

#[test]
fn label_fmt() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "foo")]
    struct Foo {
        #[span]
        pub source: Arc<NamedSource>,

        #[label("type not allowed: {name}")]
        pub span: Range<usize>,

        pub name: &'static str,
    }

    let source = Arc::new(NamedSource::new(
        "some_file.lm",
        r#"fn main() -> void {
    return 0;
}
    "#,
    ));

    assert_snapshot!(render(Foo {
        source,
        span: 13..17,
        name: "void"
    }));
}

#[test]
fn label_fmt_debug() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "foo")]
    struct Foo {
        #[span]
        pub source: Arc<NamedSource>,

        #[label("type not allowed: {name:?}")]
        pub span: Range<usize>,

        pub name: &'static str,
    }

    let source = Arc::new(NamedSource::new(
        "some_file.lm",
        r#"fn main() -> void {
    return 0;
}
"#,
    ));

    assert_snapshot!(render(Foo {
        source,
        span: 13..17,
        name: "void"
    }));
}
