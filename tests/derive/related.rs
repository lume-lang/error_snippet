use std::ops::Range;
use std::sync::Arc;

use error_snippet::NamedSource;
use error_snippet_derive::Diagnostic;
use insta::assert_snapshot;

use crate::render;

#[test]
fn single_related() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "child error")]
    struct Child {}

    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "parent error")]
    struct Parent {
        #[related]
        pub child: error_snippet::Error,
    }

    assert_snapshot!(render(Parent { child: Child {}.into() }));
}

#[test]
fn multiple_related() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "child error")]
    struct Child {}

    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "parent error")]
    struct Parent {
        #[related(collection)]
        pub children: Vec<error_snippet::Error>,
    }

    assert_snapshot!(render(Parent {
        children: vec![Child {}.into(), Child {}.into()]
    }));
}

#[test]
fn related_with_source() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "child error")]
    struct Child {
        #[span]
        pub source: Arc<NamedSource>,

        #[label("type not allowed")]
        pub span: Range<usize>,
    }

    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "parent error")]
    struct Parent {
        #[related(collection)]
        pub children: Vec<error_snippet::Error>,
    }

    let source = Arc::new(NamedSource::new(
        "some_file.lm",
        r#"fn main() -> void {
    return 0;
}
"#,
    ));

    assert_snapshot!(render(Parent {
        children: vec![Child { source, span: 13..17 }.into()]
    }));
}

#[test]
fn sourced_error_with_related() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "child error")]
    struct Child {}

    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "parent error")]
    struct Parent {
        #[related(collection)]
        pub children: Vec<error_snippet::Error>,

        #[span]
        pub source: Arc<NamedSource>,

        #[label("type not allowed")]
        pub span: Range<usize>,
    }

    let source = Arc::new(NamedSource::new(
        "some_file.lm",
        r#"fn main() -> void {
    return 0;
}
"#,
    ));

    assert_snapshot!(render(Parent {
        children: vec![Child {}.into()],
        source,
        span: 13..17
    }));
}

#[test]
fn multiple_related_with_source() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "child error")]
    struct Child {
        #[span]
        pub source: Arc<NamedSource>,

        #[label("type not allowed")]
        pub span: Range<usize>,
    }

    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "parent error")]
    struct Parent {
        #[related(collection)]
        pub children: Vec<error_snippet::Error>,
    }

    let source = Arc::new(NamedSource::new(
        "some_file.lm",
        r#"fn main() -> void {
    return 0;
}
"#,
    ));

    assert_snapshot!(render(Parent {
        children: vec![
            Child {
                source: source.clone(),
                span: 13..17
            }
            .into(),
            Child {
                source: source.clone(),
                span: 24..30
            }
            .into()
        ]
    }));
}
