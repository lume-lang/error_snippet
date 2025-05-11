use crate::Diagnostic;

pub mod graphical;

pub use graphical::*;

/// Defines a trait for rendering diagnostics to a formatter.
pub trait Renderer {
    /// Renders the diagnostic to a string buffer.
    fn render(&mut self, diagnostic: &dyn Diagnostic) -> String {
        let mut buffer = String::new();
        self.render_fmt(&mut buffer, diagnostic).unwrap();

        buffer
    }

    /// Renders the diagnostic to the given formatter.
    fn render_fmt(
        &mut self,
        f: &mut impl std::fmt::Write,
        diagnostic: &dyn Diagnostic,
    ) -> std::fmt::Result;
}
