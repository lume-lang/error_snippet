use error_snippet::{Diagnostic, GraphicalRenderer, Renderer};

mod derive;
mod renderer;

fn render(diagnostic: impl Diagnostic) -> String {
    let mut renderer = GraphicalRenderer::new();
    renderer.use_colors = false;

    renderer.render(&diagnostic).to_string()
}
