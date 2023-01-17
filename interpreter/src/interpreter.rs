use crate::ast::{self, Eval, Var};
use crate::priede_std::io::{print, printnl};

pub fn func_return(input: &ast::FunCall) -> ast::ValueNode {
    if input.id == "aa" {
        return ast::ValueNode::Number(5);
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
