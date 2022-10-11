use nx::program::Program;

#[test]
fn assert() {
    let program = Program::new("lang/spec/assert.nx".into(), ".cache".into());
    Program::run(program, "zig".into());
}
