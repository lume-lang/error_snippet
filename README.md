# `error_snippet`

`error_snippet` is a simple diagnostic library for Rust. It's made to
get out of the way when all you want is a, some-what nice looking, renderer
for diagnostics.

It let's you define your own error types using attributes, which makes the
entire process very quick and maintainable. When rendered to the terminal,
it will end up looking something like this:

![Example view of how `error_snippet` renders diagnostics](./examples/source_slice.png)

```rs
use std::{ops::Range, sync::Arc};
use error_snippet_derive::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(
    message = "whoops! an error occured!",
    code = "blunder::your::fault",
    help = "seems to be an issue of skill"
)]
struct PebcakError {
    /// This defines the source code we're printing snippets of,
    /// when the error is rendered. It can also be a [`Arc<String>`] if you
    /// don't care about the file name.
    #[span]
    pub source: Arc<dyn Source>,

    /// Labels need to have some span to define where the snippet to start
    /// and end. Labels are highlighted with arrows and, if enabled, colors!
    #[label("error occured here")]
    pub span: Range<usize>,
}
```

# MSRV

This crate requires rustc 1.85.0 or later.

# License

`error_snippet` is released as open-source software under the [MIT License](./LICENSE).
