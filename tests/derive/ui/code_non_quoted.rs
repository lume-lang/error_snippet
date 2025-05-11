use error_snippet_derive::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(message = "some help", code = E5132)]
struct Foo {}

fn main() {}
