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
    //Number(i32),
    Int(i32),
    Nat(u32),
    Long(i128),
    LongNat(u128),
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
    fn pop_int(&self) -> i32;
    fn pop_long(&self) -> i128;
    fn pop_long_nat(&self) -> u128;
    fn pop_nat(&self) -> u32;
    fn pop_str(&self) -> String;
}
impl Eval for ValueNode {
    fn eval(&self) -> ValueNode {
        match &self {
            ValueNode::Int(value) => return ValueNode::Int(*value),
            ValueNode::Nat(value) => return ValueNode::Nat(*value),
            ValueNode::Long(value) => return ValueNode::Long(*value),
            ValueNode::LongNat(value) => return ValueNode::LongNat(*value),
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
            ValueNode::Int(value) => write!(f, "{:}", value),
            ValueNode::Nat(value) => write!(f, "{:}", value),
            ValueNode::Long(value) => write!(f, "{:}", value),
            ValueNode::LongNat(value) => write!(f, "{:}", value),
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
    fn pop_int(&self) -> i32 {
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

    fn pop_long(&self) -> i128 {
        match &self {
            ValueNode::Long(value) => return *value,
            _ => todo!(),
        }
    }

    fn pop_long_nat(&self) -> u128 {
        match &self {
            ValueNode::LongNat(value) => return *value,
            _ => todo!(),
        }
    }

    fn pop_nat(&self) -> u32 {
        match &self {
            ValueNode::Nat(value) => return *value,
            _ => todo!(),
        }
    }
}
