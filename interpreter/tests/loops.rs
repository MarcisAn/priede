use interpreter;


#[test]
fn simple_loop() {
    interpreter::interpret("../examples/tests/loops/simple_loop.pr".to_string(), 3);
}


#[test]
fn while_loop() {
    interpreter::interpret("../examples/tests/loops/while_loop.pr".to_string(), 3);
}