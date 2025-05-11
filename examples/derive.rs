use std::{ops::Range, sync::Arc};

use error_snippet::{GraphicalRenderer, NamedSource, Renderer, Source};
use error_snippet_derive::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(
    message = "application error occured",
    code = "error::skill_issue",
    help = "seems to be an issue of skill"
)]
struct ApplicationError {
    #[span]
    pub source: Arc<dyn Source>,

    #[label("error occured here")]
    pub span: Range<usize>,
}

fn main() {
    let source = Arc::new(NamedSource::new(
        "std/array.lm",
        r#"fn foo() -> void {
    bar();
}"#,
    ));

    let error = ApplicationError {
        source,
        span: 23..29,
    };

    let mut renderer = GraphicalRenderer::new();
    renderer.render_stderr(&error).unwrap();
}
