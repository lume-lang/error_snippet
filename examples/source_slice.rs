use std::sync::Arc;

use error_snippet::{
    GraphicalRenderer, Help, Label, NamedSource, Renderer, Severity, SimpleDiagnostic, SourceLocation, Suggestion,
};

fn main() {
    let source = Arc::new(NamedSource::new(
        "std/array.lm",
        r#"namespace std

/// Defines an ordered, integer-indexed container which can be used as a container for values. Since the `Arra` type
/// is a generic type, all values within the array must be of the same type.
///
/// While they can be created like any other class using `new Array()`, they are most often created
/// implicitly by using the array literal syntax. To create an array, surround a comma-separated list of
/// values with square brackets:
///
/// ```
/// # Create an empty array
/// []
///
/// # Create an array with values
/// [1, 2, 3]
/// ```
///
/// Both of these literals create an array of type `Array<Int32>`, or more generally, `[Int32]`. To explicitlydefine
/// the type of the array, you can declare it's type like so:
///
/// ```
/// let a: [Int32] = [10, 20, 30];
/// ```
class builtin Array<T>
{
    /// Allocates a new array with enough space for at least `capacity` elements.
    ///
    /// When creating an array with a set capacity, it's length will still be zero.
    pub fn with_capacity(capacity: UInt64) -> Array<T> {
        let array = Array<T>::new();

        /// Allocate the minimum amount of capacity in the array.
        array.reserve(capacity);

        return true;
    }
}"#,
    ));

    let message = SimpleDiagnostic::new("mismatched types")
        .with_code("E0308")
        .with_severity(Severity::Error)
        .with_label(Label::error(
            Some(source.clone()),
            1198..1209,
            "expected `Array<T>`, found `Boolean`",
        ))
        .with_label(Label::note(
            Some(source.clone()),
            1041..1049,
            "expected type `Array<T>` found here",
        ))
        .with_help(
            Help::new("consider casting to `Array<T>`").with_suggestions([Suggestion::insert(
                SourceLocation::new(source.clone(), 1209),
                " as Array<T>",
            )]),
        );

    let mut renderer = GraphicalRenderer::new();
    renderer.highlight_source = true;
    renderer.render_stderr(&message).unwrap();
}
