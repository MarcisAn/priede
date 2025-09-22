use interpreter;
#[test]
fn array_def() {
    let returns = interpreter::interpret(
        "../examples/tests/arrays/array_def.pr".to_string(),
        false,
        false,
        false,
        true
    );
    let expected = vec![celsium::vm::StackValue::String { value: "ābols".to_string() }];
    assert_eq!(expected, returns.testing_stack);
}

#[test]
fn array_redef() {
    let returns = interpreter::interpret(
        "../examples/tests/arrays/redef.pr".to_string(),
        false,
        false,
        false,
        true
    );
    let expected = vec![
        celsium::vm::StackValue::String { value: "ābols".to_string() },
        celsium::vm::StackValue::String { value: "kivi".to_string() }
    ];
    assert_eq!(expected, returns.testing_stack);
}

#[test]
fn array_length() {
    let returns = interpreter::interpret(
        "../examples/tests/arrays/array_length.pr".to_string(),
        false,
        false,
        false,
        true
    );
    let expected = vec![celsium::vm::StackValue::Int { value: 4 }];
    assert_eq!(expected, returns.testing_stack);
}

#[test]
fn print_all() {
    let returns = interpreter::interpret(
        "../examples/tests/arrays/print_all.pr".to_string(),
        false,
        false,
        false,
        true
    );
    let expected = vec![
        celsium::vm::StackValue::String { value: "ābols".to_string() },
        celsium::vm::StackValue::String { value: "bumbieris".to_string() },
        celsium::vm::StackValue::String { value: "banāns".to_string() },
        celsium::vm::StackValue::String { value: "apelsīns".to_string() }
    ];
    assert_eq!(expected, returns.testing_stack);
}
