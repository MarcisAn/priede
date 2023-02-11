use crate::ast::{Eval, ValueNode};
use priede_std;

pub fn run_function(name: &str, args: Vec<ValueNode>) -> Result<ValueNode, bool> {
    let return_value: ValueNode = match name {
        "drukāt" => ValueNode::String(priede_std::print(args[0].eval().to_string())),
        "drukātJr" => ValueNode::String(priede_std::printnl(args[0].eval().to_string())),
        "ievade" => ValueNode::String(priede_std::ievade(args[0].eval().to_string())),
        _ => return Err(false),
    };
    Ok(return_value)
}
