use celsium::{ block::{ Block, TextSpan }, bytecode::BINOP };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ util, Compiler };

use super::parse_ast;

fn span_of(node: AstNode) -> TextSpan {
    let (line, col_start, length) = util::get_node_position_and_span_unicode(node);
    TextSpan { line, col_start, length }
}

fn parse_binary_operands(node: AstNode, compiler: &mut Compiler, block: &mut Block) -> String {
    if node.child(0).get_symbol().name == "(" {
        parse_ast(node.child(1), compiler, block);
        parse_ast(node.child(3), compiler, block);
        node.child(2).get_value().unwrap().to_string()
    } else {
        parse_ast(node.child(0), compiler, block);
        parse_ast(node.child(2), compiler, block);
        node.child(1).get_value().unwrap().to_string()
    }
}

pub fn comparisons(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title == "comp_s" {
        let span = span_of(node);
        let sign = parse_binary_operands(node, compiler, block);

        let left_type = compiler.typestack.clone().peek_level(1);
        let right_type = compiler.typestack.clone().peek_level(0);

        let checked_type = match sign.as_str() {
            "=" => compiler.typestack.binop(BINOP::Eq),
            ">" => compiler.typestack.binop(BINOP::LargerThan),
            ">=" => compiler.typestack.binop(BINOP::LargerOrEq),
            "<" => compiler.typestack.binop(BINOP::LessThan),
            "<=" => compiler.typestack.binop(BINOP::LessOrEq),
            "!=" => compiler.typestack.binop(BINOP::NotEq),
            _ => panic!("Neatpazīts salīdzinājuma simbols"),
        };

        if checked_type == None {
            compiler.add_error(
                crate::errors::CompileTimeErrorType::BinopNotPossible {
                    left: left_type.unwrap(),
                    right: right_type.unwrap(),
                },
                node
            );
        }

        match sign.as_str() {
            "=" => block.binop(BINOP::Eq, span),
            ">" => block.binop(BINOP::LargerThan, span),
            ">=" => block.binop(BINOP::LargerOrEq, span),
            "<" => block.binop(BINOP::LessThan, span),
            "<=" => block.binop(BINOP::LessOrEq, span),
            "!=" => block.binop(BINOP::NotEq, span),
            _ => panic!("Neatpazīts salīdzinājuma simbols"),
        }
    } else if title == "un" || title == "vai" || title == "xvai" {
        let span = span_of(node);

        if node.child(0).get_symbol().name == "(" {
            parse_ast(node.child(1), compiler, block);
            parse_ast(node.child(3), compiler, block);
        } else {
            parse_ast(node.child(0), compiler, block);
            parse_ast(node.child(1), compiler, block);
        }

        let op = match title {
            "un" => BINOP::And,
            "vai" => BINOP::Or,
            "xvai" => BINOP::Xor,
            _ => unreachable!(),
        };

        block.binop(op.clone(), span);
        compiler.typestack.binop(op);
    }
}
