use interpreter;

#[test]
fn array_def() {
    interpreter::interpret("../examples/tests/if/array_def.pr".to_string(), 3);
}

#[test]
fn array_redef() {
    interpreter::interpret("../examples/tests/if/redef.pr".to_string(), 3);
}

#[test]
fn array_length() {
    interpreter::interpret("../examples/tests/if/array_length.pr".to_string(), 3);
}

#[test]
fn print_all() {
    interpreter::interpret("../examples/tests/if/print_all.pr".to_string(), 3);
}