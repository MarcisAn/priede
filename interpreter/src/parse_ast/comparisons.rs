use celsium::{ block::{ Block, TextSpan }, bytecode::BINOP };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ util, Compiler };

use super::parse_ast;

pub fn comparisons(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title == "comp_s" {
        let (line, col_start, length) = util::get_node_position_and_span_unicode(node);

        let sign = if node.child(0).get_symbol().name == "(" {
            parse_ast(node.child(1), compiler, block);
            parse_ast(node.child(3), compiler, block);
            node.child(2).get_value().unwrap()
        } else {
            parse_ast(node.child(0), compiler, block);
            parse_ast(node.child(2), compiler, block);
            node.child(1).get_value().unwrap()
        };

        let checked_type = match sign {
            "=" => compiler.typestack.binop(BINOP::Eq),
            ">" => compiler.typestack.binop(BINOP::LargerThan),
            ">=" => compiler.typestack.binop(BINOP::LargerOrEq),
            "<" => compiler.typestack.binop(BINOP::LessThan),
            "<=" => compiler.typestack.binop(BINOP::LessOrEq),
            "!=" => compiler.typestack.binop(BINOP::NotEq),
            _ => panic!("Neatpazīts salīdzinājuma simbols"),
        };
        if checked_type.is_none() {
            println!("not possible");
            // errors::binop_not_possible(side_1_type, side_2_type, &mut compiler.helper, node);
        }
        match sign {
            "=" => block.binop(BINOP::Eq, TextSpan { line, col_start, length }),
            ">" => block.binop(BINOP::LargerThan, TextSpan { line, col_start, length }),
            ">=" => block.binop(BINOP::LargerOrEq, TextSpan { line, col_start, length }),
            "<" => block.binop(BINOP::LessThan, TextSpan { line, col_start, length }),
            "<=" => block.binop(BINOP::LessOrEq, TextSpan { line, col_start, length }),
            "!=" => block.binop(BINOP::NotEq, TextSpan { line, col_start, length }),
            _ => panic!("Neatpazīts salīdzinājuma simbols"),
        }
    } else if title == "un" {
        let node_span = node.get_total_position_and_span().unwrap();

        if node.child(0).get_symbol().name == "(" {
            parse_ast(node.child(1), compiler, block);
            parse_ast(node.child(3), compiler, block);
        } else {
            parse_ast(node.child(0), compiler, block);
            parse_ast(node.child(1), compiler, block);
        }
        block.binop(BINOP::And, TextSpan {
            line: node_span.0.line,
            col_start: node_span.1.index,
            length: node_span.1.length,
        });
        compiler.typestack.binop(BINOP::And);
    } else if title == "vai" {
        let node_span = node.get_total_position_and_span().unwrap();

        if node.child(0).get_symbol().name == "(" {
            parse_ast(node.child(1), compiler, block);
            parse_ast(node.child(3), compiler, block);
        } else {
            parse_ast(node.child(0), compiler, block);
            parse_ast(node.child(1), compiler, block);
        }
        block.binop(BINOP::Or, TextSpan {
            line: node_span.0.line,
            col_start: node_span.1.index,
            length: node_span.1.length,
        });
        compiler.typestack.binop(BINOP::Or);
    } else if title == "xvai" {
        let node_span = node.get_total_position_and_span().unwrap();

        if node.child(0).get_symbol().name == "(" {
            parse_ast(node.child(1), compiler, block);
            parse_ast(node.child(3), compiler, block);
        } else {
            parse_ast(node.child(0), compiler, block);
            parse_ast(node.child(1), compiler, block);
        }
        block.binop(BINOP::Xor, TextSpan {
            line: node_span.0.line,
            col_start: node_span.1.index,
            length: node_span.1.length,
        });
        compiler.typestack.binop(BINOP::Xor);
    }
}
