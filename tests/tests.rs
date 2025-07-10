use error_snippet::{Diagnostic, GraphicalRenderer, Renderer};

mod derive;
mod renderer;

fn render(diagnostic: impl Diagnostic) -> String {
    let mut renderer = GraphicalRenderer::new();
    renderer.use_colors = false;

    owo_colors::with_override(false, || renderer.render(&diagnostic).unwrap().to_string())
}
