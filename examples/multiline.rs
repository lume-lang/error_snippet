use std::sync::Arc;

use error_snippet::{GraphicalRenderer, Label, NamedSource, Renderer, Severity, SimpleDiagnostic, WithSource};

fn main() {
    let source = Arc::new(NamedSource::new(
        "README.md",
        r#"def five = match () in {
    () => 5,
    () => "5",
}

def six =
    five
    + 1"#,
    ));

    let message = SimpleDiagnostic::new("incompatible types")
        .with_code("E0308")
        .with_severity(Severity::Error)
        .with_label(Label::error(
            None,
            11..48,
            "The values are outputs of this match expression",
        ))
        .with_label(Label::help(None, 29..31, "This has type of Void"))
        .with_label(Label::warning(None, 35..36, "This has type of Str"))
        .with_help("Outputs of match expressions must coerce to the same type")
        .with_source(source);

    let mut renderer = GraphicalRenderer::new();
    renderer.highlight_source = true;
    renderer.render_stderr(&message).unwrap();
}
