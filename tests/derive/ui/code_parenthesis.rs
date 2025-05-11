use error_snippet_derive::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(message = "some help", code("E5123"))]
struct Foo {}

fn main() {}
