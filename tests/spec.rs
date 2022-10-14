use nx::program::Program;

fn assert_panic(path: &str) {
    let program = Program::new(".cache".into());
    let result = Program::run(program, path.into(), "zig".into());
    assert!(result.is_err());
}

#[test]
fn assert() {
    let program = Program::new(".cache".into());
    Program::run(program, "lang/spec/assert.nx".into(), "zig".into()).unwrap()
}

#[test]
fn panic_unused_expression() {
    assert_panic("lang/spec/panic-unused-expression-result.nx");
}

#[test]
fn panic_variable_not_found() {
    assert_panic("lang/spec/panic-variable-not-found.nx");
}
