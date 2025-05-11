use error_snippet_derive::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(message = "some help", severity = help)]
struct Foo {}

fn main() {}
