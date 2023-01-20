use crate::ast::{self, Eval, Pop, ValueNode, Var};
use crate::ast_parser::parse_function;
use crate::hime_redist::symbols::SemanticElementTrait;
use crate::priede_std::io::{print, printnl};
use hime_redist::ast::AstNode;

pub fn func_return(input: &ast::FunCall) -> ast::ValueNode {
    if input.id == "aa" {
        return ast::ValueNode::Int(5);
    } else if input.id == "drukāt" {
        //print!("{}", input.args[0].eval());
        print(format!("{}", input.args[0].eval()));
        return ast::ValueNode::None("".to_string());
    } else if input.id == "drukātJr" || input.id == "drukātjr" {
        //println!("");
        //print!("{}", input.args[0].eval());
        printnl(format!("{}", input.args[0].eval()));
        return ast::ValueNode::None("".to_string());
    } else {
        return ast::ValueNode::None("".to_string());
    }
}
pub fn id_return(input: String) -> ast::ValueNode {
    let mut value: ast::ValueNode = ast::ValueNode::None("".to_string());
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

pub fn define_variable(input: &Var) -> ast::ValueNode {
    unsafe {
        VARIABLES.push(ast::Var {
            id: input.id.clone(),
            var_type: input.var_type.clone(),
            value: input.value.clone(),
        });
    }
    return ast::ValueNode::None("".to_string());
}
//TODO: merge next two functions
pub fn arithemtics_int(input: AstNode) -> ValueNode {
    let operation = input.children().at(1).get_value().unwrap();
    let left = parse_function(input.children().at(0));
    let right = parse_function(input.children().at(2));

    let mut left_type = "";
    let mut right_type = "";

    let mut left_node = parse_function(input.children().at(0));
    let mut right_node = parse_function(input.children().at(2));

    match &left {
        ValueNode::Int(value) => left_type = "int",
        ValueNode::Nat(value) => left_type = "nat",
        ValueNode::Long(value) => left_type = "long",
        ValueNode::LongNat(value) => left_type = "longnat",
        ValueNode::String(value) => left_type = "string",
        ValueNode::Bool(value) => left_type = "bool",
        _ => todo!(),
    };
    match &right {
        ValueNode::Int(value) => right_type = "int",
        ValueNode::Nat(value) => right_type = "nat",
        ValueNode::Long(value) => right_type = "long",
        ValueNode::LongNat(value) => right_type = "longnat",
        ValueNode::String(value) => right_type = "string",
        ValueNode::Bool(value) => right_type = "bool",
        _ => todo!(),
    };
    if right_type == "longnat" || left_type == "longnat" {
        return arithemtics(&operation, left_node, right_node, "longnat");
    } else if right_type == "long" || left_type == "long" {
        return arithemtics(&operation, left_node, right_node, "long");
    } else if right_type == "nat" || left_type == "nat" {
        return arithemtics(&operation, left_node, right_node, "nat");
    } else if right_type == "int" || left_type == "int" {
        return arithemtics(&operation, left_node, right_node, "int");
    } else {
        return ValueNode::None("".to_string());
    }
}
pub fn arithemtics(
    operation: &str,
    left: ValueNode,
    right: ValueNode,
    typ: &str,
) -> ast::ValueNode {
    let mut res: ValueNode;
    if operation == "+" {
        match typ {
            "int" => res = ValueNode::Int(left.pop_int() + right.pop_int()).eval(),
            "nat" => res = ValueNode::Nat(left.pop_nat() + right.pop_nat()).eval(),
            "long" => res = ValueNode::Long(left.pop_long() + right.pop_long()).eval(),
            "longnat" => {
                res = ValueNode::LongNat(left.pop_long_nat() + right.pop_long_nat()).eval()
            }
            &_ => todo!(),
        }
    } else if operation == "-" {
        match typ {
            "int" => res = ValueNode::Int(left.pop_int() - right.pop_int()).eval(),
            "nat" => res = ValueNode::Nat(left.pop_nat() - right.pop_nat()).eval(),
            "long" => res = ValueNode::Long(left.pop_long() - right.pop_long()).eval(),
            "longnat" => {
                res = ValueNode::LongNat(left.pop_long_nat() - right.pop_long_nat()).eval()
            }
            &_ => todo!(),
        }
    } else if operation == "*" {
        match typ {
            "int" => res = ValueNode::Int(left.pop_int() * right.pop_int()).eval(),
            "nat" => res = ValueNode::Nat(left.pop_nat() * right.pop_nat()).eval(),
            "long" => res = ValueNode::Long(left.pop_long() * right.pop_long()).eval(),
            "longnat" => {
                res = ValueNode::LongNat(left.pop_long_nat() * right.pop_long_nat()).eval()
            }
            &_ => todo!(),
        }
    } else if operation == "/" {
        match typ {
            "int" => res = ValueNode::Int(left.pop_int() / right.pop_int()).eval(),
            "nat" => res = ValueNode::Nat(left.pop_nat() / right.pop_nat()).eval(),
            "long" => res = ValueNode::Long(left.pop_long() / right.pop_long()).eval(),
            "longnat" => {
                res = ValueNode::LongNat(left.pop_long_nat() / right.pop_long_nat()).eval()
            }
            &_ => todo!(),
        }
    } else {
        res = ValueNode::None("".to_string());
    }

    return res;
}
