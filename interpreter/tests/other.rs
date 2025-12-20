use interpreter;

#[test]
fn not() {
    let returns = interpreter::interpret(
        "../examples/tests/not_operator.pr".to_string(),
        false,
        false,
        false,
        true
    );
    let expected = vec![
        celsium::vm::StackValue::Bool { value: false },
        celsium::vm::StackValue::Bool { value: true },
        celsium::vm::StackValue::Bool { value: true },
        celsium::vm::StackValue::Bool { value: false },
        celsium::vm::StackValue::Bool { value: false },
        celsium::vm::StackValue::Bool { value: false },
        celsium::vm::StackValue::Bool { value: true },
        celsium::vm::StackValue::Bool { value: true },
        celsium::vm::StackValue::Bool { value: false },

    ];
    assert_eq!(expected, returns.testing_stack);
}
