use interpreter;
use celsium::vm::StackValue;

#[test]
fn cikls6() {
    let returns = interpreter::interpret(
        "../examples/clevercode_tasks/cikls6.pr".to_string(),
        false,
        false,
        false,
        true
    );
    let expected = vec![
        celsium::vm::StackValue::Int { value: 5 },
        celsium::vm::StackValue::Int { value: 5 },
        celsium::vm::StackValue::Int { value: 5 },
        celsium::vm::StackValue::Int { value: 5 },
        celsium::vm::StackValue::Int { value: 5 },
        celsium::vm::StackValue::Int { value: 5 },

    ];
    assert_eq!(expected, returns.testing_stack);
}
#[test]
fn cipvirk() {
    let returns = interpreter::interpret(
        "../examples/clevercode_tasks/cipvirk.pr".to_string(),
        false,
        false,
        false,
        true
    );
    let expected = vec![
        celsium::vm::StackValue::Int { value: 8 },
    ];
    assert_eq!(expected, returns.testing_stack);
}
#[test]
fn summa2() {
    let returns = interpreter::interpret(
        "../examples/clevercode_tasks/summa2.pr".to_string(),
        false,
        false,
        false,
        true
    );
    let expected = vec![
        celsium::vm::StackValue::Int { value: 7496561 },
    ];
    assert_eq!(expected, returns.testing_stack);
}
