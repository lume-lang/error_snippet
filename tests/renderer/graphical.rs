use std::sync::Arc;

use error_snippet::{Help, Label, NamedSource, Severity, SimpleDiagnostic, SourceLocation, SourceRange, Suggestion};
use insta::assert_snapshot;

use crate::render;

#[test]
fn only_message() {
    let message = SimpleDiagnostic::new("mismatched types");

    assert_snapshot!(render(message));
}

#[test]
fn with_code() {
    let message = SimpleDiagnostic::new("mismatched types").with_code("E0308");

    assert_snapshot!(render(message));
}

#[test]
fn with_severity_error() {
    let message = SimpleDiagnostic::new("mismatched types").with_severity(Severity::Error);

    assert_snapshot!(render(message));
}

#[test]
fn with_severity_warning() {
    let message = SimpleDiagnostic::new("mismatched types").with_severity(Severity::Warning);

    assert_snapshot!(render(message));
}

#[test]
fn with_severity_info() {
    let message = SimpleDiagnostic::new("mismatched types").with_severity(Severity::Info);

    assert_snapshot!(render(message));
}

#[test]
fn with_severity_note() {
    let message = SimpleDiagnostic::new("mismatched types").with_severity(Severity::Note);

    assert_snapshot!(render(message));
}

#[test]
fn with_severity_help() {
    let message = SimpleDiagnostic::new("mismatched types").with_severity(Severity::Help);

    assert_snapshot!(render(message));
}

#[test]
fn with_related_single() {
    let related = std::io::Error::other("failed to read file");
    let message = SimpleDiagnostic::new("could not compile").add_related(related);

    assert_snapshot!(render(message));
}

#[test]
fn with_related_multiple() {
    let related1 = std::io::Error::other("failed to read file");
    let related2 = std::io::Error::other("permission denied");

    let message = SimpleDiagnostic::new("could not compile")
        .add_related(related1)
        .add_related(related2);

    assert_snapshot!(render(message));
}

#[test]
fn with_related_nested() {
    let related1 = SimpleDiagnostic::new("permission denied");
    let related2 = SimpleDiagnostic::new("failed to read file").add_related(related1);
    let message = SimpleDiagnostic::new("could not compile").add_related(related2);

    assert_snapshot!(render(message));
}

#[test]
fn with_related_labelled() {
    let source = Arc::new(NamedSource::new(
        "src/test.lm",
        "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;",
    ));

    let related =
        SimpleDiagnostic::new("failed to read file").with_label(Label::new(Some(source), 15..20, "labelled message"));

    let message = SimpleDiagnostic::new("could not compile").add_related(related);

    assert_snapshot!(render(message));
}

#[test]
fn with_related_labelled_multiple() {
    let source = Arc::new(NamedSource::new(
        "src/test.lm",
        "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;",
    ));

    let related1 = SimpleDiagnostic::new("failed to read file").with_label(Label::new(
        Some(source.clone()),
        15..20,
        "labelled message 1",
    ));

    let related2 = SimpleDiagnostic::new("permission denied").with_label(Label::new(
        Some(source.clone()),
        30..35,
        "labelled message 2",
    ));

    let message = SimpleDiagnostic::new("could not compile")
        .add_related(related1)
        .add_related(related2);

    assert_snapshot!(render(message));
}

#[test]
fn with_label_single() {
    let source = Arc::new(NamedSource::new(
        "src/test.lm",
        "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;",
    ));

    let message =
        SimpleDiagnostic::new("mismatched types").with_label(Label::new(Some(source), 15..20, "labelled message"));

    assert_snapshot!(render(message));
}

#[test]
fn with_unnamed_source() {
    let source = Arc::new("let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;");

    let message =
        SimpleDiagnostic::new("mismatched types").with_label(Label::new(Some(source), 15..20, "labelled message"));

    assert_snapshot!(render(message));
}

#[test]
fn with_label_multiple() {
    let source = Arc::new(NamedSource::new(
        "src/test.lm",
        "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;",
    ));

    let message = SimpleDiagnostic::new("mismatched types")
        .with_label(Label::new(Some(source.clone()), 15..20, "labelled message 1"))
        .with_label(Label::new(Some(source.clone()), 30..35, "labelled message 2"));

    assert_snapshot!(render(message));
}

#[test]
fn with_label_different_files() {
    let source1 = Arc::new(NamedSource::new(
        "src/file1.lm",
        "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;",
    ));

    let source2 = Arc::new(NamedSource::new(
        "src/file2.lm",
        "let f = 1;\nlet g = 2;\nlet h = a + b;\nlet i = c * 2;\nlet j = (d + 3) * 2;",
    ));

    let message = SimpleDiagnostic::new("mismatched types")
        .with_label(Label::new(Some(source1.clone()), 15..20, "labelled message 1"))
        .with_label(Label::new(Some(source2.clone()), 30..35, "labelled message 2"));

    assert_snapshot!(render(message));
}

#[test]
fn with_label_severity_error() {
    let source = Arc::new(NamedSource::new(
        "src/test.lm",
        "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;",
    ));

    let message =
        SimpleDiagnostic::new("mismatched types").with_label(Label::error(Some(source), 15..20, "labelled message"));

    assert_snapshot!(render(message));
}

#[test]
fn with_label_severity_warning() {
    let source = Arc::new(NamedSource::new(
        "src/test.lm",
        "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;",
    ));

    let message =
        SimpleDiagnostic::new("mismatched types").with_label(Label::warning(Some(source), 15..20, "labelled message"));

    assert_snapshot!(render(message));
}

#[test]
fn with_label_severity_info() {
    let source = Arc::new(NamedSource::new(
        "src/test.lm",
        "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;",
    ));

    let message =
        SimpleDiagnostic::new("mismatched types").with_label(Label::info(Some(source), 15..20, "labelled message"));

    assert_snapshot!(render(message));
}

#[test]
fn with_label_severity_note() {
    let source = Arc::new(NamedSource::new(
        "src/test.lm",
        "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;",
    ));

    let message =
        SimpleDiagnostic::new("mismatched types").with_label(Label::note(Some(source), 15..20, "labelled message"));

    assert_snapshot!(render(message));
}

#[test]
fn with_label_severity_help() {
    let source = Arc::new(NamedSource::new(
        "src/test.lm",
        "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;",
    ));

    let message =
        SimpleDiagnostic::new("mismatched types").with_label(Label::help(Some(source), 15..20, "labelled message"));

    assert_snapshot!(render(message));
}

#[test]
fn with_help_single() {
    let message = SimpleDiagnostic::new("mismatched types").with_help("did you check your syntax?");

    assert_snapshot!(render(message));
}

#[test]
fn with_help_multiple() {
    let message = SimpleDiagnostic::new("mismatched types")
        .with_help("did you check your syntax?")
        .with_help("it would really help if you did...");

    assert_snapshot!(render(message));
}

#[test]
fn with_help_newlines() {
    let message =
        SimpleDiagnostic::new("mismatched types").with_help("expected type `Array<T>`\n   found type `Boolean`");

    assert_snapshot!(render(message));
}

#[test]
fn with_help_suggestion_delete() {
    let source = Arc::new(NamedSource::new(
        "src/test.lm",
        r#"fn foo() -> Boolean {
    return false as Boolean;
}"#,
    ));

    let message = SimpleDiagnostic::new("unnecessary cast").with_help(
        Help::new("remove unnecessary cast here")
            .with_suggestion(Suggestion::delete(SourceRange::new(source.clone(), 38..49))),
    );

    assert_snapshot!(render(message));
}

#[test]
fn with_help_suggestion_replace() {
    let source = Arc::new(NamedSource::new(
        "src/test.lm",
        r#"fn foo() -> Boolean {
    return fals;
}"#,
    ));

    let message = SimpleDiagnostic::new("invalid value").with_help(
        Help::new("did you mean `false`?")
            .with_suggestion(Suggestion::replace(SourceRange::new(source.clone(), 33..37), "false")),
    );

    assert_snapshot!(render(message));
}

#[test]
fn with_help_suggestion_insert() {
    let source = Arc::new(NamedSource::new(
        "src/test.lm",
        r#"fn foo() -> Boolean {
    return 0;
}"#,
    ));

    let message = SimpleDiagnostic::new("mismatched types").with_help(
        Help::new("cast the value `0` to a `Boolean`").with_suggestion(Suggestion::insert(
            SourceLocation::new(source.clone(), 34),
            " as Boolean",
        )),
    );

    assert_snapshot!(render(message));
}

#[test]
fn with_help_suggestion_multiple() {
    let source = Arc::new(NamedSource::new(
        "src/test.lm",
        r#"fn foo() -> Boolean {
    return (false);
}"#,
    ));

    let message = SimpleDiagnostic::new("unnecessary parenthesis").with_help(
        Help::new("remove unnecessary parenthesis here")
            .with_suggestion(Suggestion::delete(SourceRange::new(source.clone(), 33..34)))
            .with_suggestion(Suggestion::delete(SourceRange::new(source.clone(), 39..40))),
    );

    assert_snapshot!(render(message));
}

#[test]
fn with_help_suggestion_different_lines() {
    let source = Arc::new(NamedSource::new(
        "src/test.lm",
        r#"fn foo() -> Boolean {
    return false;
}"#,
    ));

    let message = SimpleDiagnostic::new("unnecessary parenthesis").with_help(
        Help::new("remove unnecessary parenthesis here")
            .with_suggestion(Suggestion::replace(
                SourceRange::new(source.clone(), 12..19),
                "CoolBoolean",
            ))
            .with_suggestion(Suggestion::delete(SourceRange::new(source.clone(), 33..38))),
    );

    assert_snapshot!(render(message));
}
