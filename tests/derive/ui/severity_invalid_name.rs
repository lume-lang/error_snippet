use error_snippet_derive::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(message = "some help", severity = Bug)]
struct Foo {}

fn main() {}
