use crate::{
    ast::{Eval, Pop, ValueNode},
    interpreter::print_error,
};
use colored::*;
use priede_std;

pub fn run_function(name: &str, args: Vec<ValueNode>, line: usize) -> Result<ValueNode, bool> {
    let return_value: ValueNode = match name {
        "drukāt" => ValueNode::String(priede_std::print(args[0].eval().to_string())),
        "drukātJr" => ValueNode::String(priede_std::printnl(args[0].eval().to_string())),
        "ievade" => ValueNode::String(priede_std::ievade(args[0].eval().to_string())),
        "kāp" => ValueNode::Int(priede_std::kapinat(
            args[0].eval().pop_int(),
            args[1].eval().pop_int(),
        )),
        "kvadrātsak" => ValueNode::Int(priede_std::kvadratsakne(args[0].eval().pop_int())),
        "test" => test_fn(args, line),
        _ => return Err(false),
    };
    Ok(return_value)
}
fn test_fn(args: Vec<ValueNode>, line: usize) -> ValueNode {
    let mut eq = false;
    if args[0].pop_type() == "int" && args[1].pop_type() == "int" {
        eq = priede_std::comp_int(args[0].pop_int(), args[1].pop_int());
    } else if args[0].pop_type() == "string" && args[1].pop_type() == "string" {
        eq = priede_std::comp_str(args[0].pop_str(), args[1].pop_str());
    } else if args[0].pop_type() == "bool" && args[1].pop_type() == "bool" {
        eq = priede_std::comp_bool(args[0].pop_bool(), args[1].pop_bool());
    } else {
        print_error(line, "Vērtības nav salīdzināmas".to_owned());
        return ValueNode::None("".to_string());
    }
    if eq {
        print!("{}", "VIENĀDI".green());
    } else {
        print!("{}", "NAV VIENĀDI".red());
    }
    return ValueNode::Bool(eq);
}
