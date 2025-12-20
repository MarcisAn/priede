use interpreter;
use celsium::vm::StackValue;

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

#[test]
fn break_continue() {
    let returns = interpreter::interpret(
        "../examples/tests/loops/break_continue.pr".to_string(),
        false,
        false,
        false,
        true
    );
    let expected = vec![
        StackValue::Int { value: 0 },
        StackValue::Int { value: 1 },
        StackValue::Int { value: 2 },
        StackValue::String { value: "Beigas".to_string() },
        StackValue::Int { value: 1 },
        StackValue::Int { value: 3 },
        StackValue::Int { value: 4 },
        StackValue::Int { value: 5 },
        StackValue::String { value: "Beigas".to_string() },
    ];
    assert_eq!(expected, returns.testing_stack);
}
