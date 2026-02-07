use interpreter;

#[test]
fn var_def() {
    let returns = interpreter::interpret(
        "../examples/tests/variables/var_def.pr".to_string(),
        false,
        false,
        false,
        true
    );
    let expected = vec![
        celsium::vm::StackValue::Int { value: 2 },
        celsium::vm::StackValue::String { value: "Sveika, pasaule!".to_string() },
        celsium::vm::StackValue::Bool { value: true },
        celsium::vm::StackValue::Bool { value: false },
        celsium::vm::StackValue::String { value: "Būla mainīgais a ir patiess".to_string() },
        celsium::vm::StackValue::Int { value: 3 },
        celsium::vm::StackValue::Int { value: 4 },
        celsium::vm::StackValue::Int { value: 4 },
        celsium::vm::StackValue::Int { value: 3 },
    ];
    assert_eq!(expected, returns.testing_stack);

}
