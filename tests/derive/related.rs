use std::{ops::Range, sync::Arc};

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
        pub children: Vec<Child>,
    }

    assert_snapshot!(render(Parent {
        children: vec![Child {}]
    }));
}

#[test]
fn multiple_related() {
    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "child error")]
    struct Child {}

    #[derive(Debug, Diagnostic)]
    #[diagnostic(message = "parent error")]
    struct Parent {
        #[related]
        pub children: Vec<Child>,
    }

    assert_snapshot!(render(Parent {
        children: vec![Child {}, Child {}]
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
        #[related]
        pub children: Vec<Child>,
    }

    let source = Arc::new(NamedSource::new(
        "some_file.lm",
        r#"fn main() -> void {
    return 0;
}
"#,
    ));

    assert_snapshot!(render(Parent {
        children: vec![Child {
            source,
            span: 13..17
        }]
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
        #[related]
        pub children: Vec<Child>,
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
            },
            Child {
                source: source.clone(),
                span: 24..30
            }
        ]
    }));
}
