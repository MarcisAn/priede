use core::fmt;

//struct FuncDef {
//    id: String,
//    args: Vec<FuncArg>,
//}
//struct FuncArg {
//    arg_type: String,
//    arg_name: String,
//}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum ValueNode {
    Number(i32),
    String(String),
    None(String),
    FunCall(FunCall),
    Id(String),
    VarDef(Box<Var>),
    Bool(bool),
}
#[derive(Debug)]
pub enum AcrionNode {}
pub trait Eval {
    fn eval(&self) -> ValueNode;
}
pub trait Pop {
    fn pop_number(&self) -> i32;
    fn pop_str(&self) -> String;
}
impl Eval for ValueNode {
    fn eval(&self) -> ValueNode {
        match &self {
            ValueNode::Number(value) => return ValueNode::Number(*value),
            ValueNode::None(_) => ValueNode::None("".to_string()),
            ValueNode::FunCall(value) => crate::interpreter::func_return(value),
            ValueNode::String(value) => return ValueNode::String(value.to_string()),
            ValueNode::Id(value) => crate::interpreter::id_return(value.to_string()),
            ValueNode::VarDef(value) => crate::interpreter::define_variable(value),
            ValueNode::Bool(value) => ValueNode::Bool(*value),
        }
    }
}

impl fmt::Display for ValueNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ValueNode::Number(value) => write!(f, "{:}", value),
            ValueNode::None(_) => write!(f, "NULL"),
            ValueNode::FunCall(value) => write!(f, "{:?}", crate::interpreter::func_return(value)),
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
    fn pop_number(&self) -> i32 {
        match &self {
            ValueNode::Number(value) => return *value,
            _ => todo!(),
        }
    }
    fn pop_str(&self) -> String {
        match &self {
            ValueNode::String(value) => return String::from(value),
            _ => todo!(),
        }
    }
}
