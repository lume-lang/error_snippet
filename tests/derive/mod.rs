mod cause;
mod code;
mod help;
mod label;
mod message;
mod related;
mod severity;

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/derive/ui/*.rs");
}
