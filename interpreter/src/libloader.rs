use crate::ast::{Eval, Pop, ValueNode};
use priede_std;

pub fn run_function(name: &str, args: Vec<ValueNode>) -> Result<ValueNode, bool> {
    let return_value: ValueNode = match name {
        "drukāt" => ValueNode::String(priede_std::print(args[0].eval().to_string())),
        "drukātJr" => ValueNode::String(priede_std::printnl(args[0].eval().to_string())),
        "ievade" => ValueNode::String(priede_std::ievade(args[0].eval().to_string())),
        "kāp" => ValueNode::Int(priede_std::kapinat(
            args[0].eval().pop_int(),
            args[1].eval().pop_int(),
        )),
        "kvadrātsak" => ValueNode::Int(priede_std::kvadratsakne(args[0].eval().pop_int())),
        _ => return Err(false),
    };
    Ok(return_value)
}
