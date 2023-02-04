use cinner::{cin, cout};
use std::io::{self, stdin, BufRead, Write};

use crate::ast::{self, Eval, Pop, ValueNode, Var};
use crate::ast_parser::parse_function;
use crate::hime_redist::symbols::SemanticElementTrait;
use crate::priede_std::io::{print, printnl};
use colored::*;
use hime_redist::ast::AstNode;
use std::process;
pub fn print_error(line: usize, msg: String) {
    println!(
        "{}",
        format!(
            "Kļūda rindiņā {}: \n{}",
            line.to_string(),
            msg.to_string().bright_magenta()
        )
        .red()
    );
    //process::exit(1);
}
fn input_2(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map(|x| x.trim_end().to_owned())
}
pub fn func_return(input: &ast::FunCall) -> ast::ValueNode {
    if input.id == "drukāt" {
        print(format!("{}", input.args[0].eval()));
        return ast::ValueNode::None("".to_string());
    } else if input.id == "drukātJr" || input.id == "drukātjr" {
        printnl(format!("{}", input.args[0].eval()));
        return ast::ValueNode::None("".to_string());
    } else if input.id == "ievade" {
        let mut user_input = input_2("").unwrap();
        return ast::ValueNode::String(user_input);
    } else {
        return ast::ValueNode::None("".to_string());
    }
}
pub fn id_return(input: String) -> ast::ValueNode {
    let mut value: ast::ValueNode = ast::ValueNode::None("".to_string());
    unsafe {
        for i in &VARIABLES {
            if i.id == input {
                value = i.value.clone()
            }
        }
    }
    return value;
}
static mut VARIABLES: Vec<Var> = Vec::new();

pub fn define_variable(id: String, var_type: String, value: ValueNode) -> ValueNode {
    let mut overwritten_value_node: ValueNode = value.clone();

    if &var_type == "NUM" {
        let mut overwritten_value;
        match &value {
            ValueNode::Int(value) => overwritten_value = *value as i32,
            ValueNode::Nat(value) => overwritten_value = *value as i32,
            ValueNode::Long(value) => overwritten_value = *value as i32,
            ValueNode::LongNat(value) => overwritten_value = *value as i32,
            ValueNode::Bool(value) => overwritten_value = *value as i32,
            ValueNode::String(value) => overwritten_value = value.parse::<i32>().unwrap(),
            _ => todo!(),
        }
        overwritten_value_node = ValueNode::Int(overwritten_value);
    }
    if &var_type == "LONG" {
        let mut overwritten_value;
        match &value {
            ValueNode::Int(value) => overwritten_value = *value as i64,
            ValueNode::Nat(value) => overwritten_value = *value as i64,
            ValueNode::Long(value) => overwritten_value = *value as i64,
            ValueNode::LongNat(value) => overwritten_value = *value as i64,
            ValueNode::Bool(value) => overwritten_value = *value as i64,
            ValueNode::String(value) => overwritten_value = value.parse::<i64>().unwrap(),
            _ => todo!(),
        }
        overwritten_value_node = ValueNode::Long(overwritten_value);
    }
    if &var_type == "NAT" {
        let mut overwritten_value;
        match &value {
            ValueNode::Int(value) => overwritten_value = *value as u32,
            ValueNode::Nat(value) => overwritten_value = *value as u32,
            ValueNode::Long(value) => overwritten_value = *value as u32,
            ValueNode::LongNat(value) => overwritten_value = *value as u32,
            ValueNode::Bool(value) => overwritten_value = *value as u32,
            ValueNode::String(value) => overwritten_value = value.parse::<u32>().unwrap(),

            _ => todo!(),
        }
        overwritten_value_node = ValueNode::Nat(overwritten_value);
    }
    if &var_type == "LONGNAT" {
        let mut overwritten_value;
        match &value {
            ValueNode::Int(value) => overwritten_value = *value as u64,
            ValueNode::Nat(value) => overwritten_value = *value as u64,
            ValueNode::Long(value) => overwritten_value = *value as u64,
            ValueNode::LongNat(value) => overwritten_value = *value as u64,
            ValueNode::Bool(value) => overwritten_value = *value as u64,
            ValueNode::String(value) => overwritten_value = value.parse::<u64>().unwrap(),
            _ => todo!(),
        }
        overwritten_value_node = ValueNode::LongNat(overwritten_value);
    }

    let var = ast::Var {
        id: id.clone(),
        value: overwritten_value_node,
        var_type: var_type,
    };
    unsafe {
        VARIABLES.push(var.clone());
    }
    return ValueNode::None("".to_string());
}
pub fn id_assign(id: String, value: ValueNode, line: usize) {
    let mut mutable_index = 0;
    let mut iter = 0;
    let mut found = false;
    unsafe {
        for i in &VARIABLES {
            if i.id == id {
                mutable_index = iter;
                found = true;
            }
            iter += 1;
        }
        if found {
            VARIABLES[mutable_index].value = value;
        }
    }
    if !found {
        print_error(line, format!("Mainīgais \"{}\" nav atrasts", id));
    }
}
//TODO: merge next two functions
pub fn arithemtics_int(input: AstNode) -> Result<ValueNode, String> {
    let operation = input.children().at(1).get_value().unwrap();
    let left = parse_function(input.children().at(0));
    let right = parse_function(input.children().at(2));

    let mut left_type = "";
    let mut right_type = "";

    let left_node = parse_function(input.children().at(0));
    let right_node = parse_function(input.children().at(2));

    match &left {
        ValueNode::Int(_) => left_type = "int",
        ValueNode::Nat(_) => left_type = "nat",
        ValueNode::Long(_) => left_type = "long",
        ValueNode::LongNat(_) => left_type = "longnat",
        ValueNode::String(_) => left_type = "string",
        ValueNode::Bool(_) => left_type = "bool",
        _ => todo!(),
    };
    match &right {
        ValueNode::Int(_) => right_type = "int",
        ValueNode::Nat(_) => right_type = "nat",
        ValueNode::Long(_) => right_type = "long",
        ValueNode::LongNat(_) => right_type = "longnat",
        ValueNode::String(_) => right_type = "string",
        ValueNode::Bool(_) => right_type = "bool",
        _ => todo!(),
    };
    if left_type != right_type {
        return Err("Aritmētiskās darbības locekļu datu tipi nav vienādi".to_string());
    } else {
        if right_type == "longnat" || left_type == "longnat" {
            return Ok(arithemtics(&operation, left_node, right_node, "longnat"));
        } else if right_type == "long" || left_type == "long" {
            return Ok(arithemtics(&operation, left_node, right_node, "long"));
        } else if right_type == "nat" || left_type == "nat" {
            return Ok(arithemtics(&operation, left_node, right_node, "nat"));
        } else if right_type == "int" || left_type == "int" {
            return Ok(arithemtics(&operation, left_node, right_node, "int"));
        } else {
            return Err("nav iespējams veikt aritmētisko darbību".to_string());
        }
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
pub fn compare(left: ValueNode, right: ValueNode, action: String) -> Result<bool, String> {
    let mut left_type;
    let mut right_type;
    match &left {
        ValueNode::Int(_) => left_type = "int",
        ValueNode::Nat(_) => left_type = "nat",
        ValueNode::Long(_) => left_type = "long",
        ValueNode::LongNat(_) => left_type = "longnat",
        ValueNode::String(_) => left_type = "string",
        ValueNode::Bool(_) => left_type = "bool",
        _ => todo!(),
    };
    match &right {
        ValueNode::Int(_) => right_type = "int",
        ValueNode::Nat(_) => right_type = "nat",
        ValueNode::Long(_) => right_type = "long",
        ValueNode::LongNat(_) => right_type = "longnat",
        ValueNode::String(_) => right_type = "string",
        ValueNode::Bool(_) => right_type = "bool",
        _ => todo!(),
    };
    //print!("{} {}", left_type, right_type);
    if left_type != right_type {
        return Err("salīdzināmie datu tipi nav vienādi".to_string());
    } else {
        if action == "=" {
            if left == right {
                return Ok(true);
            } else {
                return Ok(false);
            }
        } else if action == ">" {
            if left > right {
                return Ok(true);
            } else {
                return Ok(false);
            }
        } else if action == ">=" {
            if left >= right {
                return Ok(true);
            } else {
                return Ok(false);
            }
        } else if action == "<" {
            if left < right {
                return Ok(true);
            } else {
                return Ok(false);
            }
        } else if action == "<=" {
            if left <= right {
                return Ok(true);
            } else {
                return Ok(false);
            }
        } else if action == "!=" {
            if left != right {
                return Ok(true);
            } else {
                return Ok(false);
            }
        } else {
            return Err(format!("\"{}\"nav atpazīta salīdzināšanas darbība", action));
        }
    }
}
