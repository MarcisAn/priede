use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ hime, util };

#[derive(Debug, Clone)]
pub struct FormatingContext<'a> {
    indentation_level: usize,
    indentation: &'a str,
}

struct LineComment {
    line: usize,
    whitespace: usize,
    text: String,
    
}

pub fn format(code: String, print_ast: bool) -> String {
    let parse_res = hime::priede::parse_string(code.clone());

    if parse_res.errors.errors.len() > 0 {
        return code;
    }

    let ast = parse_res.get_ast();
    let root = ast.get_root();
    if print_ast {
        util::print_ast(root);
    }

    let mut line_comments: Vec<LineComment> = vec![];

    for (i, line) in code.lines().into_iter().enumerate() {
        let comment_start = line.find("//");
        if comment_start.is_some() {
            let comment_text = line.split_at(comment_start.unwrap()).1.to_string();
            let whitespace = comment_start.unwrap() - line[..comment_start.unwrap()].trim_end().len();
            line_comments.push(LineComment {
                line: i,
                whitespace: whitespace,
                text: comment_text,
            });
        }
    }

    let mut context = FormatingContext { indentation: "\t", indentation_level: 0 };
    let formated_from_ast = code_from_ast(root, &mut context).0;
    let mut result: String = "".to_string();
    for (i, line) in formated_from_ast.lines().into_iter().enumerate(){
        for comment in &line_comments{
            if comment.line == i {
                result += &format!("{}{}{}\n", line," ".repeat(comment.whitespace), comment.text);
                continue;
            }
        }
        result += line;
        result += "\n";
    }
    return result;
}

fn code_from_ast<'a>(node: AstNode, context: &mut FormatingContext) -> (String, usize) {
    //return the formated string and number of lines
    let title = node.get_symbol().name;
    if title == "var_def" {
        return (
            format!(
                "{} {} : {}",
                node.child(0).get_value().unwrap(),
                node.child(1).get_value().unwrap(),
                code_from_ast(node.child(2), context).0
            ),
            1,
        );
    } else if title == "ID" {
        return (node.get_value().unwrap().to_string(), 0);
    } else if title == "NUMBER" {
        if node.children_count() > 0 {
            //id assign
            return (
                node.child(0).get_value().unwrap().to_string() + node.child(1).get_symbol().name,
                1,
            );
        } else {
            return (node.get_value().unwrap().to_string(), 0);
        }
    } else if title == "BOOL" {
        let value = if node.child(0).get_symbol().name == "TRUE" { "PATIESS" } else { "NEPATIESS" };
        return (value.to_string(), 0);
    } else if title == "STRING" {
        return (node.get_value().unwrap().to_string(), 0);
    } else if title == "return_st" {
        return (format!("atgriest {}", code_from_ast(node.child(0), context).0), 1);
    } else if title == "array" {
        let mut elements = "".to_string();
        for element in node.children() {
            elements += element.get_value().unwrap();
            if element != node.children().into_iter().last().unwrap() {
                elements += "; ";
            }
        }
        return (format!("[{}]", elements), 0);
    } else if title == "id_assign" {
        return (
            format!(
                "{} {} {}",
                code_from_ast(node.child(0), context).0,
                node.child(1).get_symbol().name,
                code_from_ast(node.child(2), context).0
            ),
            1,
        );
    } else if title == "func_call" {
        let mut arguments = String::new();
        for arg in node.child(1).children() {
            arguments += &code_from_ast(arg, context).0;
            if arg != node.child(1).children().into_iter().last().unwrap() {
                arguments += "; ";
            }
        }
        return (format!("{}({})", node.child(0).get_value().unwrap(), arguments), 1);
    } else if title == "comp_s" {
        return (
            format!(
                "{} {} {}",
                code_from_ast(node.child(0), context).0,
                node.child(1).get_symbol().name,
                code_from_ast(node.child(2), context).0
            ),
            0,
        );
    } else if title == "plus" {
        return (
            format!(
                "{} + {}",
                code_from_ast(node.child(0), context).0,
                code_from_ast(node.child(2), context).0
            ),
            0,
        );
    } else if title == "minus" {
        return (
            format!(
                "{} - {}",
                code_from_ast(node.child(0), context).0,
                code_from_ast(node.child(2), context).0
            ),
            0,
        );
    } else if title == "reiz" {
        return (
            format!(
                "{} * {}",
                code_from_ast(node.child(0), context).0,
                code_from_ast(node.child(2), context).0
            ),
            0,
        );
    } else if title == "dal" {
        return (
            format!(
                "{} / {}",
                code_from_ast(node.child(0), context).0,
                code_from_ast(node.child(2), context).0
            ),
            0,
        );
    } else if title == "atlik" {
        return (
            format!(
                "{} % {}",
                code_from_ast(node.child(0), context).0,
                code_from_ast(node.child(2), context).0
            ),
            0,
        );
    } else if title == "un" {
        return (
            format!(
                "{} un {}",
                code_from_ast(node.child(0), context).0,
                code_from_ast(node.child(1), context).0
            ),
            0,
        );
    } else if title == "vai" {
        return (
            format!(
                "{} vai {}",
                code_from_ast(node.child(0), context).0,
                code_from_ast(node.child(1), context).0
            ),
            0,
        );
    } else if title == "xvai" {
        return (
            format!(
                "{} xvai {}",
                code_from_ast(node.child(0), context).0,
                code_from_ast(node.child(1), context).0
            ),
            0,
        );
    } else if title == "s_loop" {
        let condition = code_from_ast(node.child(0), context);
        let body = code_from_ast(node.child(1), context);
        return (format!("atkārtot ({}) {{\n{}\n}}", condition.0, body.0), 2 + condition.1 + body.1);
    } else if title == "w_loop" {
        let condition = code_from_ast(node.child(0), context);
        let body = code_from_ast(node.child(1), context);
        return (format!("kamēr ({}) {{\n{}\n}}", condition.0, body.0), 2 + condition.1 + body.1);
    } else if title == "block" {
        let mut block: String = "".into();
        let mut last_ending = 1;
        let mut is_first_child_node = true;
        let start = util::get_closest_node_location(node.child(0)).unwrap().line;
        let mut end = 0;
        if node.parent().is_some() {
            context.indentation_level += 1;
        }
        for statement in node.children() {
            let statement_start = util::get_closest_node_location(statement).unwrap().line;
            let parsed_statement = code_from_ast(statement, context);
            end = statement_start + parsed_statement.1 - 1;
            let mut empty_lines = statement_start - last_ending;
            last_ending = end;
            if is_first_child_node {
                empty_lines = 0;
            }
            is_first_child_node = false;
            block += &format!(
                "{}{}{}",
                "\n".repeat(empty_lines),
                context.indentation.repeat(context.indentation_level),
                parsed_statement.0
            );
        }
        if node.parent().is_some() {
            context.indentation_level -= 1;
        }
        let block_width = if end < start { 0 } else { end - start + 1 };
        return (block.to_string(), block_width);
    } else if title == "if" {
        let condition = code_from_ast(node.child(0), context);
        let body = code_from_ast(node.child(1), context);
        return (
            format!(
                "ja ({}) {{\n{}\n{}}}",
                condition.0,
                body.0,
                context.indentation.repeat(context.indentation_level)
            ),
            2 + condition.1 + body.1,
        );
    } else {
        ("".into(), 0)
    }
}
