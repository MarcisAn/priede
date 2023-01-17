//use colored::*;
use crate::print;
use core::fmt;
use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};
//struct FuncDef {
//    id: String,
//    args: Vec<FuncArg>,
//}
//struct FuncArg {
//    arg_type: String,
//    arg_name: String,
//}

#[derive(Debug, Clone)]
struct FunCall {
    id: String,
    args: Vec<ValueNode>,
}

#[derive(Debug, Clone)]
struct Var {
    id: String,
    var_type: String,
    value: ValueNode,
}

#[derive(Debug, Clone)]
enum ValueNode {
    Number(i32),
    String(String),
    None(String),
    FunCall(FunCall),
    Id(String),
    VarDef(Box<Var>),
    Bool(bool),
}
#[derive(Debug)]
enum AcrionNode {}
trait Eval {
    fn eval(&self) -> ValueNode;
}
trait Pop {
    fn pop_number(&self) -> i32;
    fn pop_str(&self) -> String;
}
fn func_return(input: &FunCall) -> ValueNode {
    if input.id == "aa" {
        return ValueNode::Number(5);
    } else if input.id == "drukāt" {
        //print!("{}", input.args[0].eval());
        print(format!("{}", input.args[0].eval()));
        return ValueNode::None("".to_string());
    } else if input.id == "drukātJr" {
        //println!("");
        //print!("{}", input.args[0].eval());
        print(format!("\n{}", input.args[0].eval()));
        return ValueNode::None("".to_string());
    } else {
        return ValueNode::None("".to_string());
    }
}
fn id_return(input: String) -> ValueNode {
    let mut value: ValueNode = ValueNode::None("".to_string());
    unsafe {
        for i in &VARIABLES {
            if i.id == input {
                match i.var_type.as_str() {
                    "NUM" => value = i.value.clone(),
                    "NATURAL" => value = i.value.clone(),
                    "LONG" => value = i.value.clone(),
                    "LONG_NAT" => value = i.value.clone(),
                    "TEXT" => value = i.value.clone(),
                    "BOOL_DEF" => value = i.value.clone(),
                    &_ => todo!(),
                }
            }
        }
    }
    return value;
}
static mut VARIABLES: Vec<Var> = Vec::new();
fn define_variable(input: &Var) -> ValueNode {
    unsafe {
        VARIABLES.push(Var {
            id: input.id.clone(),
            var_type: input.var_type.clone(),
            value: input.value.clone(),
        });
    }
    return ValueNode::None("".to_string());
}
impl Eval for ValueNode {
    fn eval(&self) -> ValueNode {
        match &self {
            ValueNode::Number(value) => return ValueNode::Number(*value),
            ValueNode::None(_) => ValueNode::None("".to_string()),
            ValueNode::FunCall(value) => func_return(value),
            ValueNode::String(value) => return ValueNode::String(value.to_string()),
            ValueNode::Id(value) => id_return(value.to_string()),
            ValueNode::VarDef(value) => define_variable(value),
            ValueNode::Bool(value) => ValueNode::Bool(*value),
        }
    }
}

impl fmt::Display for ValueNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ValueNode::Number(value) => write!(f, "{:}", value),
            ValueNode::None(_) => write!(f, "NULL"),
            ValueNode::FunCall(value) => write!(f, "{:?}", func_return(value)),
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

fn parse_function(input: AstNode<'_>) -> crate::ast_parser::ValueNode {
    if input.to_string() == "func" {
        let node = input.clone();
        let binding = node.children();
        let binding = binding.at(1);
        let args = binding.children(); //argumenti
        let id = node
            .children()
            .at(0)
            .to_string()
            .split("ID = ")
            .collect::<Vec<&str>>()[1]
            .to_string();
        let mut arguments: Vec<ValueNode> = Vec::new();
        let mut j = 0;
        while j < node.children().at(1).children().len() {
            arguments.push(parse_function(args.at(j)));
            j += 1;
        }
        return ValueNode::FunCall(FunCall {
            id: id.to_string(),
            args: arguments,
        })
        .eval();
    } else if input.to_string() == "exp_plusmin" || input.to_string() == "exp_reizdal" {
        // aritmētiskās darbības
        let left_value = parse_function(input.children().at(0)).pop_number();
        let right_value = parse_function(input.children().at(2)).pop_number();
        if input.children().at(1).to_string() == "+ = +" {
            return ValueNode::Number(left_value + right_value).eval();
        } else if input.children().at(1).to_string() == "- = -" {
            return ValueNode::Number(left_value - right_value).eval();
        } else if input.children().at(1).to_string() == "* = *" {
            return ValueNode::Number(left_value * right_value).eval();
        } else if input.children().at(1).to_string() == "/ = /" {
            return ValueNode::Number(left_value / right_value).eval();
        } else {
            return ValueNode::None("".to_string()).eval();
        }
    } else if input.to_string().starts_with("NUMBER = ") {
        return ValueNode::Number(input.get_value().unwrap().parse::<i32>().unwrap());
    } else if input.to_string() == "BOOL" {
        if input.children().at(0).to_string() == "FALSE" {
            return ValueNode::Bool(false);
        } else {
            return ValueNode::Bool(true);
        }
    } else if input.to_string().starts_with("ID = ") {
        return ValueNode::Id(input.get_value().unwrap()).eval();
    } else if input.to_string().starts_with("STRING = ") {
        let string = input.to_string().split("STRING = ").collect::<Vec<&str>>()[1].to_string();
        if string.starts_with("\"") && string.ends_with("\"") {
            return ValueNode::String(
                string.split("\"").collect::<Vec<&str>>()[1]
                    .parse::<String>()
                    .unwrap(),
            )
            .eval();
        } else if string.starts_with("\'") && string.ends_with("\'") {
            return ValueNode::String(
                string.split("\'").collect::<Vec<&str>>()[1]
                    .parse::<String>()
                    .unwrap(),
            )
            .eval();
        } else {
            return ValueNode::None("".to_string());
        }
    } else if input.to_string() == "var_def" {
        let id = input.children().at(1).get_value().unwrap();
        let var_type = input.children().at(0).to_string();

        return ValueNode::VarDef(Box::new(Var {
            id: id,
            var_type: var_type,
            value: parse_function(input.children().at(2)),
        }))
        .eval();
    } else {
        return ValueNode::None("".to_string()).eval();
    }
}
pub fn parse_ast(input: AstNode<'_>) {
    let mut i = 0;
    let children = input.children();
    while i < input.children().len() {
        parse_function(children.at(i));
        i += 1;
    }
}
