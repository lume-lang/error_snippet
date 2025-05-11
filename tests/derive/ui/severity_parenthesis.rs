use error_snippet_derive::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(message = "some help", severity(Warning))]
struct Foo {}

fn main() {}
