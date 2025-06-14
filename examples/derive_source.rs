use std::{ops::Range, sync::Arc};

use error_snippet::{GraphicalRenderer, NamedSource, Renderer};
use error_snippet_derive::Diagnostic;

#[derive(Debug, Clone)]
pub struct Location {
    pub source: Arc<NamedSource>,

    pub span: Range<usize>,
}

impl From<Location> for Arc<dyn error_snippet::Source> {
    fn from(value: Location) -> Self {
        value.source
    }
}

impl From<Location> for error_snippet::SpanRange {
    fn from(value: Location) -> Self {
        value.span.into()
    }
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    message = "application error occured",
    code = "error::skill_issue",
    help = "seems to be an issue of skill"
)]
struct ApplicationError {
    #[label(source, "error occured here")]
    pub source: Location,
}

fn main() {
    let source = Arc::new(NamedSource::new(
        "std/array.lm",
        r#"fn foo() -> void {
    bar();
}"#,
    ));

    let error = ApplicationError {
        source: Location {
            source: source.clone(),
            span: 23..29,
        },
    };

    let mut renderer = GraphicalRenderer::new();
    renderer.render_stderr(&error).unwrap();
}
