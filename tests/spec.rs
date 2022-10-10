use nx::program::Program;

#[test]
fn assert() {
    let program = Program::new("lang/spec/assert.nx".into(), ".cache".into(), "zig".into());
    Program::run(program);
}
