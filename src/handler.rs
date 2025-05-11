use crate::{Diagnostic, Renderer, Severity, SimpleDiagnostic};

/// Abstract handler type for reporting diagnostics.
///
/// Handlers are nothing more than a "store" for diagnostics, which
/// decides when to drain the diagnostics to the user.
pub trait Handler {
    /// Reports the diagnostic to the handler, without emitting it immediately.
    fn report(&mut self, diagnostic: Box<dyn Diagnostic>);

    /// Drains all the diagnostics to the console and empties the local store.
    fn drain(&mut self) -> std::fmt::Result;

    /// Reports the diagnostic to the handler and emits it immediately, along
    /// with all other stored diagnostics within the handler.
    fn report_and_drain(&mut self, diagnostic: Box<dyn Diagnostic>) -> std::fmt::Result {
        self.report(diagnostic);

        self.drain()
    }
}

/// The default diagnostic handler.
///
/// The [`DiagnosticHandler`] allows to report to the user immediately or deferred until drained,
/// and aborting upon draining an error (or worse) diagnostic.
///
/// # Examples
///
/// To use deferred reporting:
///
/// ```
/// use error_snippet::{SimpleDiagnostic, GraphicalRenderer, Handler, DiagnosticHandler};
///
/// let diagnostic = SimpleDiagnostic::new("An error occurred");
///
/// let renderer = GraphicalRenderer::new();
/// let mut handler = DiagnosticHandler::with_renderer(Box::new(renderer));
///
/// handler.report(Box::new(diagnostic));
/// ```
///
/// If not, you can drain the diagnostics immediately after reporting it:
///
/// ```
/// use error_snippet::{SimpleDiagnostic, GraphicalRenderer, Handler, DiagnosticHandler};
///
/// let diagnostic = SimpleDiagnostic::new("An error occurred");
///
/// let renderer = GraphicalRenderer::new();
/// let mut handler = DiagnosticHandler::with_renderer(Box::new(renderer));
///
/// handler.report_and_drain(Box::new(diagnostic));
/// ```
///
/// To abort upon draining an error diagnostic, use the [`DiagnosticHandler::exit_on_error()`] method:
///
/// ```
/// use error_snippet::{DiagnosticHandler, GraphicalRenderer};
///
/// let renderer = GraphicalRenderer::new();
/// let mut handler = DiagnosticHandler::with_renderer(Box::new(renderer));
/// handler.exit_on_error();
///
/// // ...
/// ```
pub struct DiagnosticHandler {
    /// Defines whether to exit upon emitting an error.
    exit_on_error: bool,

    /// Stores all the diagnostics which have been reported.
    emitted_diagnostics: Vec<Box<dyn Diagnostic>>,

    /// Defines the renderer to use when rendering the diagnostics.
    renderer: Box<dyn Renderer + Send + Sync>,
}

impl DiagnosticHandler {
    /// Creates a new empty handler.
    pub fn with_renderer(renderer: Box<dyn Renderer + Send + Sync>) -> Self {
        DiagnosticHandler {
            exit_on_error: false,
            emitted_diagnostics: Vec::new(),
            renderer,
        }
    }

    /// Enables the handler to exit upon emitting an error.
    pub fn exit_on_error(&mut self) {
        self.exit_on_error = true
    }
}

impl Handler for DiagnosticHandler {
    fn report(&mut self, diagnostic: Box<dyn Diagnostic>) {
        self.emitted_diagnostics.push(diagnostic);
    }

    fn drain(&mut self) -> std::fmt::Result {
        let mut encountered_errors = 0usize;

        for diagnostic in &self.emitted_diagnostics {
            self.renderer.render_stderr(diagnostic.as_ref())?;

            // If the diagnostic is an error, mark it down.
            if diagnostic.severity() == Severity::Error {
                encountered_errors += 1;
            }
        }

        // If we've encountered any errors,
        if encountered_errors > 0 && self.exit_on_error {
            let message = format!("aborting due to {} previous errors", encountered_errors);
            let abort_diag = Box::new(SimpleDiagnostic::new(message));

            self.renderer.render_stderr(abort_diag.as_ref())?;

            std::process::exit(1);
        }

        Ok(())
    }
}
