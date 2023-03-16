use crate::ast::{self, Eval, Pop, ValueNode, Var};
use crate::ast_parser::parse_function;
use crate::hime_redist::symbols::SemanticElementTrait;
use crate::libloader::run_function;
use colored::*;
use hime_redist::ast::AstNode;
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

pub fn func_return(input: &ast::FunCall) -> ast::ValueNode {
    let func = run_function(&input.id, input.args.clone());
    if func.is_ok() {
        return func.unwrap();
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

fn parse_error(line: usize) {
    print_error(
        line,
        "Neizdevās ievadi pārveidot vajadzīgajā datu tipā".to_string(),
    );
}
fn parseInt(value: &str, line: usize) -> i128 {
    let parsed = value.parse::<i128>();
    if parsed.is_ok() {
        return parsed.unwrap();
    } else {
        parse_error(line);
        return 0;
    }
}

pub fn define_variable(id: String, var_type: String, value: ValueNode, line: usize) -> ValueNode {
    let mut overwritten_value_node: ValueNode = value.clone();

    if &var_type == "NUM" {
        let overwritten_value;
        match &value {
            ValueNode::Int(value) => overwritten_value = *value as i128,
            ValueNode::Bool(value) => overwritten_value = *value as i128,
            ValueNode::String(value) => overwritten_value = parseInt(value, line),
            _ => todo!(),
        }
        overwritten_value_node = ValueNode::Int(overwritten_value);
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

    let left_type: &str;
    let right_type: &str;

    let left_node = parse_function(input.children().at(0));
    let right_node = parse_function(input.children().at(2));

    match &left {
        ValueNode::Int(_) => left_type = "int",
        ValueNode::String(_) => left_type = "string",
        ValueNode::Bool(_) => left_type = "bool",
        _ => todo!(),
    };
    match &right {
        ValueNode::Int(_) => right_type = "int",
        ValueNode::String(_) => right_type = "string",
        ValueNode::Bool(_) => right_type = "bool",
        _ => todo!(),
    };
    if left_type != right_type {
        return Err("Aritmētiskās darbības locekļu datu tipi nav vienādi".to_string());
    } else {
        if right_type == "int" || left_type == "int" {
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
    let res: ValueNode;
    if operation == "+" {
        match typ {
            "int" => res = ValueNode::Int(left.pop_int() + right.pop_int()).eval(),
            &_ => todo!(),
        }
    } else if operation == "-" {
        match typ {
            "int" => res = ValueNode::Int(left.pop_int() - right.pop_int()).eval(),
            &_ => todo!(),
        }
    } else if operation == "*" {
        match typ {
            "int" => res = ValueNode::Int(left.pop_int() * right.pop_int()).eval(),
            &_ => todo!(),
        }
    } else if operation == "/" {
        match typ {
            "int" => res = ValueNode::Int(left.pop_int() / right.pop_int()).eval(),
            &_ => todo!(),
        }
    } else if operation == "%" {
        match typ {
            "int" => res = ValueNode::Int(left.pop_int() % right.pop_int()).eval(),
            &_ => todo!(),
        }
    } else {
        res = ValueNode::None("".to_string());
    }

    return res;
}
pub fn compare(left: ValueNode, right: ValueNode, action: String) -> Result<bool, String> {
    let left_type;
    let right_type;
    match &left {
        ValueNode::Int(_) => left_type = "int",
        ValueNode::String(_) => left_type = "string",
        ValueNode::Bool(_) => left_type = "bool",
        _ => todo!(),
    };
    match &right {
        ValueNode::Int(_) => right_type = "int",
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
