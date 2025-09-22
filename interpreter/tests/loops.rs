use interpreter;

#[test]
fn simple_loop() {
    let returns = interpreter::interpret(
        "../examples/tests/loops/simple_loop.pr".to_string(),
        false,
        false,
        false,
        true
    );
    let expected = vec![
        celsium::vm::StackValue::String { value: "Teksts tiks izprintēts 3 reizes".to_string() },
        celsium::vm::StackValue::String { value: "Teksts tiks izprintēts 3 reizes".to_string() },
        celsium::vm::StackValue::String { value: "Teksts tiks izprintēts 3 reizes".to_string() },
        celsium::vm::StackValue::String { value: "Teksts tiks izprintēts 3 reizes".to_string() },
        celsium::vm::StackValue::String { value: "Teksts tiks izprintēts 3 reizes".to_string() },
        celsium::vm::StackValue::String { value: "Teksts tiks izprintēts 3 reizes".to_string() }
    ];
    assert_eq!(expected, returns.testing_stack);
}

#[test]
fn while_loop() {
    let returns = interpreter::interpret(
        "../examples/tests/loops/while_loop.pr".to_string(),
        false,
        false,
        false,
        true
    );
    let expected = vec![
        celsium::vm::StackValue::String { value: "Teksts tiks printēts 5 reizes".to_string() },
        celsium::vm::StackValue::String { value: "Teksts tiks printēts 5 reizes".to_string() },
        celsium::vm::StackValue::String { value: "Teksts tiks printēts 5 reizes".to_string() },
        celsium::vm::StackValue::String { value: "Teksts tiks printēts 5 reizes".to_string() },
        celsium::vm::StackValue::String { value: "Teksts tiks printēts 5 reizes".to_string() },
    ];
    assert_eq!(expected, returns.testing_stack);
}
