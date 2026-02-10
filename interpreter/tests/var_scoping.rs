use interpreter;

#[test]
fn var_scoping() {
    let returns = interpreter::interpret(
        "../examples/tests/variables/var_scoping.pr".to_string(),
        false,
        false,
        false,
        true
    );
    let expected = vec![
        celsium::vm::StackValue::Int { value: 4 },
        celsium::vm::StackValue::Int { value: 9 },
        celsium::vm::StackValue::Int { value: 5 },
        celsium::vm::StackValue::Int { value: 7 },
        celsium::vm::StackValue::Int { value: 6 },

    ];
    assert_eq!(expected, returns.testing_stack);

}
