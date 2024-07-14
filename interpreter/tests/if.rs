use interpreter;

#[test]
fn basic_if() {
    interpreter::interpret("../examples/tests/if/basic_if.pr".to_string(), 3,false);
}

#[test]
fn comparisons() {
    interpreter::interpret("../examples/tests/if/comparisons.pr".to_string(), 3,false);
}

#[test]
fn exp_chaining() {
    interpreter::interpret("../examples/tests/if/exp_chaining.pr".to_string(), 3,false);
}

#[test]
fn if_else() {
    interpreter::interpret("../examples/tests/if/if_else.pr".to_string(), 3,false);
}