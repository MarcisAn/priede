use std::{collections::LinkedList, fmt};

use hime_redist::text::TextPosition;

#[derive(Clone)]
pub struct BytecodeItem {
    pub(crate) typ: BytecodeItemType,
    pub(crate) position: Option<TextPosition>,
}
#[derive(Debug, Clone, PartialEq)]

pub enum VarTypes {
    Number,
    String,
    Bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BytecodeItemType {
    Add, //math
    Subtract,
    Multiply,
    Divide,
    Remainder,
    PushNumber {
        value_as_string: String,
    }, //constants
    PushString {
        value: String,
    },
    PushBool {
        value: bool,
    },
    EndIf {
        matched: bool,
    },
    IfExp,
    Equals,
    GreaterThan,
    GreaterEqual,
    LessThan,
    LessEqual,
    NotEqual,
    And,
    Or,
    ExclusiveOr,
    FunctionCall {
        name: String,
    },
    VarDef {
        name: String,
        typ: VarTypes,
    },
    MutateVar {
        name: String,
    },
    LoadVar {
        name: String,
    },
    StartSimpleLoop,
    EndSimpleLoop,
    JumpTo {
        jump_to: usize,
    },
    StartWhileLoop {
        matched: bool,
    },
    JumpToIfFalse {
        jump_to: usize,
    },
    InvertStackItem,
    FuncDef {
        name: String,
        jump_to: usize,
        len: usize,
    },
    EndFuncDef {
        matched: bool,
    },
}
