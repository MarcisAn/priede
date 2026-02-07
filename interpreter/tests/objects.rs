use celsium::vm::{ ObjectField, StackValue };
use interpreter;

#[test]
fn objects() {
    let returns = interpreter::interpret(
        "../examples/tests/objects/objects.pr".to_string(),
        false,
        false,
        false,
        true
    );
    let expected = vec![
        StackValue::Object {
            value: vec![
                ObjectField {
                    name: "suga".to_string(),
                    value: StackValue::String { value: "priede".to_string() },
                },
                ObjectField { name: "augstums".to_string(), value: StackValue::Int { value: 4 } },
                ObjectField { name: "diametrs".to_string(), value: StackValue::Int { value: 4 } }
            ],
        },
        StackValue::Array {
            value: vec![
                StackValue::Object {
                    value: vec![
                        ObjectField {
                            name: "suga".to_string(),
                            value: StackValue::String { value: "priede".to_string() },
                        },
                        ObjectField {
                            name: "augstums".to_string(),
                            value: StackValue::Int { value: 4 },
                        },
                        ObjectField {
                            name: "diametrs".to_string(),
                            value: StackValue::Int { value: 4 },
                        },
                    ],
                },
                StackValue::Object {
                    value: vec![
                        ObjectField {
                            name: "suga".to_string(),
                            value: StackValue::String { value: "egle".to_string() },
                        },
                        ObjectField {
                            name: "augstums".to_string(),
                            value: StackValue::Int { value: 6 },
                        },
                        ObjectField {
                            name: "diametrs".to_string(),
                            value: StackValue::Int { value: 5 },
                        },
                    ],
                },
            ],
        },
        StackValue::Object {
            value: vec![
                ObjectField {
                    name: "suga".to_string(),
                    value: StackValue::String { value: "priede".to_string() },
                },
                ObjectField { name: "augstums".to_string(), value: StackValue::Int { value: 4 } },
                ObjectField { name: "diametrs".to_string(), value: StackValue::Int { value: 4 } }
            ],
        },
        StackValue::String { value: "priede".to_string() },
        StackValue::String { value: "priede".to_string() },
        StackValue::String { value: "priede".to_string() },
        StackValue::String { value: "egle".to_string() }
    ];
    assert_eq!(expected, returns.testing_stack);
}
