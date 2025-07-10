use error_snippet::{DiagnosticHandler, GraphicalRenderer, Handler};
use error_snippet_derive::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(message = "failed to read file")]
struct IoError {}

fn main() {
    let error = IoError {};

    let renderer = GraphicalRenderer::new();
    let mut handler = DiagnosticHandler::with_renderer(Box::new(renderer));
    handler.exit_on_error();

    if let Err(err) = handler.report_and_drain(error.into()) {
        eprintln!("{err}");
    }
}
