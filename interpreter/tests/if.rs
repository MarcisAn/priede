use interpreter;

#[test]
fn basic_if() {
    let returns = interpreter::interpret(
        "../examples/tests/if/basic_if.pr".to_string(),
        false,
        false,
        false,
        true
    );
    let expected = vec![
        celsium::vm::StackValue::String { value: "Divi ir vienāds ar divi".to_string() },
        celsium::vm::StackValue::String { value: "Divi plus divi ir vienāds ar četri".to_string() },
    ];
    assert_eq!(expected, returns.testing_stack);
}

#[test]
fn comparisons() {
    let returns = interpreter::interpret(
        "../examples/tests/if/comparisons.pr".to_string(),
        false,
        false,
        false,
        true
    );
    let expected = vec![
        celsium::vm::StackValue::String { value: "Trīs ir lielāks par divi".to_string() },
        celsium::vm::StackValue::String { value: "Viens ir mazāks par divi".to_string() },
        celsium::vm::StackValue::String { value: "Pieci nav vienāds ar divi".to_string() },

    ];
    assert_eq!(expected, returns.testing_stack);
}

#[test]
fn exp_chaining() {
    let returns = interpreter::interpret(
        "../examples/tests/if/exp_chaining.pr".to_string(),
        false,
        false,
        false,
        true
    );
    let expected = vec![
        celsium::vm::StackValue::String { value: "Trīs ir lielāks par divi un pieci ir mazāks par septiņi".to_string() },
        celsium::vm::StackValue::String { value: "Viens ir mazāks par divi vai trīs lielāks par viens".to_string() },
        celsium::vm::StackValue::String { value: "Viens ir mazāks par divi, bet trīs nav mazāks par viens".to_string() },

    ];
    assert_eq!(expected, returns.testing_stack);
}

#[test]
fn if_else() {
    let returns = interpreter::interpret(
        "../examples/tests/if/if_else.pr".to_string(),
        false,
        false,
        false,
        true
    );
    let expected = vec![
        celsium::vm::StackValue::String { value: "Divi ir vienāds ar divi".to_string() },
        celsium::vm::StackValue::String { value: "Trīs nav lielāks par 5".to_string() },

    ];
    assert_eq!(expected, returns.testing_stack);
}
