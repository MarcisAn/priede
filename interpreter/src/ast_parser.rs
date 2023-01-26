use crate::ast::{self, Pop};
use crate::ast::{Eval, ValueNode};
use crate::interpreter::{
    arithemtics, arithemtics_int, compare, define_variable, id_assign, print_error,
};
use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};

pub fn parse_function(input: AstNode<'_>) -> ast::ValueNode {
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
        let act = arithemtics_int(input.clone());
        if act.is_err() {
            let line = input.children().at(1).get_position().unwrap().line;
            print_error(line, act.err().unwrap());
            return ast::ValueNode::None("".to_string());
        } else {
            return act.unwrap();
        }
    } else if input.to_string().starts_with("NUMBER = ") {
        //19
        if input.get_value().unwrap().len() >= 9 {
            return ast::ValueNode::Long(input.get_value().unwrap().parse::<i64>().unwrap());
        } else if input.get_value().unwrap().len() >= 19 {
            return ast::ValueNode::LongNat(input.get_value().unwrap().parse::<u64>().unwrap());
        } else {
            return ast::ValueNode::Int(input.get_value().unwrap().parse::<i32>().unwrap());
        }
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
        let value = parse_function(input.children().at(2));
        //print!("{}", input.children().at(2));
        //print!("{}", var_type);
        define_variable(id, var_type, value);
        return ast::ValueNode::None("".to_string());
    } else if input.to_string() == "block" {
        let mut i = 0;
        while i < input.children().len() {
            parse_function(input.children().at(i));
            i += 1;
        }
        return ast::ValueNode::None("".to_string()).eval();
    } else if input.to_string() == "if" {
        let comp = parse_function(input.children().at(0));
        if comp != ValueNode::None("".to_string()) {
            if input.children().at(2).get_value().unwrap() == "citādi" {
                if comp.pop_bool() {
                    parse_function(input.children().at(1));
                } else {
                    parse_function(input.children().at(3));
                }
            } else {
                if comp.pop_bool() {
                    parse_function(input.children().at(1));
                }
            }
        }

        return ast::ValueNode::None("".to_string()).eval();
    } else if input.to_string() == "comp_s" {
        let left = parse_function(input.children().at(0));
        let right = parse_function(input.children().at(2));
        let comp = compare(left, right, input.children().at(1).get_value().unwrap());
        if comp.is_err() {
            let line = input.children().at(1).get_position().unwrap().line;
            print_error(line, comp.err().unwrap());
            return ast::ValueNode::None("".to_string());
        } else {
            if comp.unwrap() {
                return ast::ValueNode::Bool(true);
            } else {
                return ast::ValueNode::Bool(false);
            }
        }
    } else if input.to_string() == "id_asign" {
        let sign = input.children().at(1).get_value().unwrap();
        let left = parse_function(input.children().at(0)).clone();
        let left_typ = left.pop_type();
        let right = parse_function(input.children().at(2));
        let assignment;
        if sign == "->" {
            assignment = parse_function(input.children().at(2));
        } else if sign == "-->" {
            assignment = arithemtics("-", left.clone(), right, left_typ);
        } else if sign == "-+>" {
            assignment = arithemtics("+", left.clone(), right, left_typ);
        } else if sign == "-*>" {
            assignment = arithemtics("*", left.clone(), right, left_typ);
        } else if sign == "-/>" {
            assignment = arithemtics("/", left.clone(), right, left_typ);
        } else {
            assignment = ValueNode::None("".to_string());
        };
        id_assign(input.children().at(0).get_value().unwrap(), assignment);
        return ast::ValueNode::None("".to_string()).eval();
    } else {
        return ast::ValueNode::None("".to_string()).eval();
    }
}
