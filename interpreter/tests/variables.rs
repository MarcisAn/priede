use interpreter;


#[test]
fn var_def() {
    interpreter::interpret("../examples/tests/variables/var_def.pr".to_string(), false, false, false, true);
}