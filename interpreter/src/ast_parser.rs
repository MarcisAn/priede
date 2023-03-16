use crate::ast::{self, FuncArg, FuncDef, Pop};
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
            line: 0, //TODO: get proper line
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
        return ast::ValueNode::Int(input.get_value().unwrap().parse::<i128>().unwrap());
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
        define_variable(
            id,
            var_type,
            value,
            input.children().at(1).get_position().unwrap().line,
        );
        return ast::ValueNode::None("".to_string());
    } else if input.to_string() == "block" {
        let mut i = 0;
        while i < input.children().len() {
            parse_function(input.children().at(i));
            i += 1;
        }
        return ast::ValueNode::None("".to_string()).eval();
    } else if input.to_string() == "if" {
        //TODO: rekursīvi iet cauri 'un' 'vai' blokiem
        let comp = parse_function(input.children().at(0));
        if comp != ValueNode::None("".to_string()) {
            if input.children().len() == 4 {
                //TODO: else if bloki
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
    } else if input.to_string() == "comp" {
        let left = parse_function(input.children().at(0).children().at(0));
        let right = parse_function(input.children().at(0).children().at(2));
        if input.children().at(0).to_string() == "un" {
            if left.pop_bool() && right.pop_bool() {
                return ValueNode::Bool(true);
            } else {
                return ValueNode::Bool(false);
            }
        } else if input.children().at(0).to_string() == "vai" {
            if left.pop_bool() || right.pop_bool() {
                return ValueNode::Bool(true);
            } else {
                return ValueNode::Bool(false);
            }
        } else {
            return parse_function(input.children().at(0));
        }
    } else if input.to_string() == "s_loop" {
        let count = parse_function(input.children().at(0)).pop_int(); //TODO: vairāk kā inti
        let mut counter = 0;
        while counter < count {
            parse_function(input.children().at(1));
            counter += 1;
        }
        return ValueNode::None("".to_string());
    } else if input.to_string() == "w_loop" {
        while true {
            if parse_function(input.children().at(0)).pop_bool() {
                parse_function(input.children().at(1));
            } else {
                break;
            }
        }
        return ValueNode::None("".to_string());
    } else if input.to_string() == "comp_s" {
        let left = parse_function(input.children().at(0));
        let right = parse_function(input.children().at(2));
        let comp = compare(left, right, input.children().at(1).get_value().unwrap());
        if comp.is_err() {
            //ja nav iespējams salīdzināt abus lielumus1
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
        } else if sign == "=-" {
            assignment = arithemtics("-", left.clone(), right, left_typ);
        } else if sign == "=+" {
            assignment = arithemtics("+", left.clone(), right, left_typ);
        } else if sign == "=*" {
            assignment = arithemtics("*", left.clone(), right, left_typ);
        } else if sign == "=/" {
            assignment = arithemtics("/", left.clone(), right, left_typ);
        } else {
            assignment = ValueNode::None("".to_string());
        };
        id_assign(
            input.children().at(0).get_value().unwrap(),
            assignment,
            input.children().at(0).get_position().unwrap().line,
        );
        return ast::ValueNode::None("".to_string()).eval();
    } else if input.to_string() == "func_def" {
        let id = input.children().at(0).get_value().unwrap();
        let mut args: Vec<FuncArg> = vec![];
        if input.children().len() == 2 {
            let func_def = FuncDef {
                id,
                args,
                body: input.children().at(1),
            };
        } else {
            let mut i = 0;
            while i < input.children().at(1).children().len() {
                args.push(FuncArg {
                    arg_type: input
                        .children()
                        .at(1)
                        .children()
                        .at(i)
                        .children()
                        .at(1)
                        .to_string(),
                    arg_name: input
                        .children()
                        .at(1)
                        .children()
                        .at(i)
                        .children()
                        .at(0)
                        .get_value()
                        .unwrap(),
                });
                i += 1;
            }
            print!("{:?}", args);
            /*
            let func_def = FuncDef {
                id,
                args,
                body: input.children().at(2),
            };*/
        }
        return ast::ValueNode::None("".to_string());
    } else {
        return ast::ValueNode::None("".to_string()).eval();
    }
}
