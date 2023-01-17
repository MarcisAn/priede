//use colored::*;
use crate::ast::Eval;
use crate::ast::{self, Pop};
use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};

fn parse_function(input: AstNode<'_>) -> ast::ValueNode {
    if input.to_string() == "func_call" {
        let node = input.clone();
        let binding = node.children();
        let binding = binding.at(1);
        let args = binding.children(); //argumenti
        let id = node.children().at(0).get_value().unwrap();
        let mut arguments: Vec<ast::ValueNode> = Vec::new();
        let mut j = 0;
        while j < node.children().at(1).children().len() {
            arguments.push(parse_function(args.at(j)));
            j += 1;
        }
        return ast::ValueNode::FunCall(ast::FunCall {
            id: id.to_string(),
            args: arguments,
        })
        .eval();
    } else if input.to_string() == "exp_plusmin" || input.to_string() == "exp_reizdal" {
        // aritmētiskās darbības
        let left_value = parse_function(input.children().at(0)).pop_number();
        let right_value = parse_function(input.children().at(2)).pop_number();
        if input.children().at(1).to_string() == "+ = +" {
            return ast::ValueNode::Number(left_value + right_value).eval();
        } else if input.children().at(1).to_string() == "- = -" {
            return ast::ValueNode::Number(left_value - right_value).eval();
        } else if input.children().at(1).to_string() == "* = *" {
            return ast::ValueNode::Number(left_value * right_value).eval();
        } else if input.children().at(1).to_string() == "/ = /" {
            return ast::ValueNode::Number(left_value / right_value).eval();
        } else {
            return ast::ValueNode::None("".to_string()).eval();
        }
    } else if input.to_string().starts_with("NUMBER = ") {
        return ast::ValueNode::Number(input.get_value().unwrap().parse::<i32>().unwrap());
    } else if input.to_string() == "BOOL" {
        if input.children().at(0).to_string() == "FALSE" {
            return ast::ValueNode::Bool(false);
        } else {
            return ast::ValueNode::Bool(true);
        }
    } else if input.to_string().starts_with("ID = ") {
        return ast::ValueNode::Id(input.get_value().unwrap()).eval();
    } else if input.to_string().starts_with("STRING = ") {
        let string = input.to_string().split("STRING = ").collect::<Vec<&str>>()[1].to_string();
        if string.starts_with("\"") && string.ends_with("\"") {
            return ast::ValueNode::String(
                string.split("\"").collect::<Vec<&str>>()[1]
                    .parse::<String>()
                    .unwrap(),
            )
            .eval();
        } else if string.starts_with("\'") && string.ends_with("\'") {
            return ast::ValueNode::String(
                string.split("\'").collect::<Vec<&str>>()[1]
                    .parse::<String>()
                    .unwrap(),
            )
            .eval();
        } else {
            return ast::ValueNode::None("".to_string());
        }
    } else if input.to_string() == "var_def" {
        let id = input.children().at(1).get_value().unwrap();
        let var_type = input.children().at(0).to_string();

        return ast::ValueNode::VarDef(Box::new(ast::Var {
            id: id,
            var_type: var_type,
            value: parse_function(input.children().at(2)),
        }))
        .eval();
    } else if input.to_string() == "block" {
        let mut i = 0;
        while i < input.children().len() {
            parse_function(input.children().at(i));
            i += 1;
        }
        return ast::ValueNode::None("".to_string()).eval();
    } else {
        return ast::ValueNode::None("".to_string()).eval();
    }
}
pub fn parse_ast(input: AstNode<'_>) {
    parse_function(input);
}
