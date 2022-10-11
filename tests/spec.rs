use nx::program::Program;

fn assert_panic(path: &str) {
    let program = Program::new(path.into(), ".cache".into());
    let result = Program::run(program, "zig".into());
    assert!(result.is_err());
}

#[test]
fn assert() {
    let program = Program::new("lang/spec/assert.nx".into(), ".cache".into());
    Program::run(program, "zig".into()).unwrap()
}

#[test]
fn panic_unused_expression() {
    assert_panic("lang/spec/panic-unused-expression-result.nx");
}

#[test]
fn panic_variable_not_found() {
    assert_panic("lang/spec/panic-variable-not-found.nx");
}
