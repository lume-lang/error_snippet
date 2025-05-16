use error_snippet::{DiagnosticHandler, Handler, Renderer, SimpleDiagnostic};

pub struct StubRenderer;

impl Renderer for StubRenderer {
    fn render_fmt(
        &mut self,
        _f: &mut error_snippet::Formatter,
        _diagnostic: &dyn error_snippet::Diagnostic,
    ) -> std::fmt::Result {
        Ok(())
    }
}

#[test]
fn drain_removes_all() {
    let renderer = Box::new(StubRenderer);
    let mut handler = DiagnosticHandler::with_renderer(renderer);

    handler.report(SimpleDiagnostic::new("foo").into());
    assert_eq!(handler.count(), 1);

    let _ = handler.drain();
    assert_eq!(handler.count(), 0);
}
