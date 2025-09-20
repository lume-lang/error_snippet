use std::sync::Arc;

use error_snippet::*;
use insta::assert_snapshot;

use crate::render;

#[test]
fn label_range_outside_range() {
    let source = Arc::new(NamedSource::new("src/test.lm", "abc"));

    let message = SimpleDiagnostic::new("failed to read file").with_label(Label::new(
        Some(source),
        256..280,
        "label_range_outside_range",
    ));

    assert_snapshot!(render(message));
}

#[test]
fn label_end_outside_range() {
    let source = Arc::new(NamedSource::new("src/test.lm", "abc"));

    let message = SimpleDiagnostic::new("failed to read file").with_label(Label::new(
        Some(source),
        0..50,
        "label_end_outside_range",
    ));

    assert_snapshot!(render(message));
}

#[test]
fn label_range_negative_length() {
    let source = Arc::new(NamedSource::new("src/test.lm", "abc"));

    let message = SimpleDiagnostic::new("failed to read file").with_label(Label::new(
        Some(source),
        3..1,
        "label_range_negative_length",
    ));

    assert_snapshot!(render(message));
}
