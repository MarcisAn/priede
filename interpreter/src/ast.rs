use core::fmt;

use hime_redist::ast::AstNode;

pub struct FuncDef<'a> {
    pub(crate) id: String,
    pub(crate) args: Vec<FuncArg>,
    pub(crate) body: AstNode<'a>,
}
#[derive(Debug, Clone)]
pub struct FuncArg {
    pub(crate) arg_type: String,
    pub(crate) arg_name: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FunCall {
    pub(crate) id: String,
    pub(crate) args: Vec<ValueNode>,
}
#[derive(Debug, Clone)]
pub struct Var {
    pub(crate) id: String,
    pub(crate) var_type: String,
    pub(crate) value: ValueNode,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ValueNode {
    //Number(i32),
    Int(i128),
    String(String),
    None(String),
    FunCall(FunCall),
    Id(String),
    Bool(bool),
}
#[derive(Debug)]
pub enum AcrionNode {}
pub trait Eval {
    fn eval(&self) -> ValueNode;
}
pub trait Pop {
    fn pop_type(&self) -> &str;
    fn pop_int(&self) -> i128;
    fn pop_str(&self) -> String;
    fn pop_bool(&self) -> bool;
}
impl Eval for ValueNode {
    fn eval(&self) -> ValueNode {
        match &self {
            ValueNode::Int(value) => return ValueNode::Int(*value),
            ValueNode::None(_) => ValueNode::None("".to_string()),
            ValueNode::FunCall(value) => crate::interpreter::func_return(value),
            ValueNode::String(value) => return ValueNode::String(value.to_string()),
            ValueNode::Id(value) => crate::interpreter::id_return(value.to_string()),
            ValueNode::Bool(value) => ValueNode::Bool(*value),
        }
    }
}

impl fmt::Display for ValueNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ValueNode::Int(value) => write!(f, "{:}", value),
            ValueNode::None(_) => write!(f, "NULL"),
            ValueNode::FunCall(value) => write!(f, "{:?}", value),
            ValueNode::String(value) => write!(f, "{}", value),
            ValueNode::Bool(value) => write!(
                f,
                "{:}",
                value
                    .to_string()
                    .replace("false", "NEPATIESS")
                    .replace("true", "PATIESS")
            ),
            _ => todo!(),
        }
    }
}
impl Pop for ValueNode {
    fn pop_bool(&self) -> bool {
        match &self {
            ValueNode::Bool(value) => return *value,
            ValueNode::Int(value) => return if value >= &1 { true } else { false },
            ValueNode::String(_) => return true,
            _ => todo!(),
        }
    }
    fn pop_int(&self) -> i128 {
        match &self {
            ValueNode::Int(value) => return *value,
            _ => todo!(),
        }
    }
    fn pop_str(&self) -> String {
        match &self {
            ValueNode::String(value) => return String::from(value),
            _ => todo!(),
        }
    }

    fn pop_type(&self) -> &str {
        let mut typ;
        match &self {
            ValueNode::Int(_) => typ = "int",
            ValueNode::String(_) => typ = "string",
            ValueNode::Bool(_) => typ = "bool",
            ValueNode::None(_) => typ = "null",
            ValueNode::FunCall(_) => todo!(),
            ValueNode::Id(_) => todo!(),
        };
        return typ;
    }
}
