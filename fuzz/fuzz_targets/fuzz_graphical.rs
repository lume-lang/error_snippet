#![no_main]

use std::{ops::Range, sync::Arc};

use arbitrary::Arbitrary;
use error_snippet::*;
use error_snippet_derive::*;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug, Clone)]
struct Source {
    pub name: String,
    pub content: String,
}

impl error_snippet::Source for Source {
    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }

    fn content(&self) -> Box<&str> {
        Box::new(&self.content)
    }
}

#[derive(Arbitrary, Debug, Clone)]
struct Location {
    pub source: Arc<Source>,
    pub span: Range<usize>,
}

impl From<Location> for Arc<dyn error_snippet::Source> {
    fn from(value: Location) -> Self {
        value.source
    }
}

impl From<Location> for error_snippet::SpanRange {
    fn from(value: Location) -> Self {
        value.span.into()
    }
}

#[derive(Arbitrary, Diagnostic, Debug)]
#[diagnostic(message = "some arbitrary diagnostic")]
struct ArbitraryDiagnostic {
    #[label(source, "error occured here")]
    pub source: Location,
}

fuzz_target!(|input: ArbitraryDiagnostic| {
    let mut renderer = GraphicalRenderer::new();
    renderer.render(&input).unwrap();
});
