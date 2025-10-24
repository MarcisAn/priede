use crate::{ errors, util, Compiler };
use celsium::{ block::{ Block, TextSpan }, bytecode::BINOP };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use super::parse_ast;

pub fn math_ops(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title == "plus" || title == "minus" || title == "reiz" || title == "dal" || title == "atlik" {
        calculate(
            match title {
                "plus" => BINOP::Add,
                "minus" => BINOP::Subtract,
                "reiz" => BINOP::Multiply,
                "dal" => BINOP::Divide,
                "atlik" => BINOP::Remainder,
                _ => panic!(),
            },
            node,
            compiler,
            block
        );
    }
}
fn calculate(binop: BINOP, node: AstNode, compiler: &mut Compiler, block: &mut Block) {
    if node.child(0).get_symbol().name == "(" {
        parse_ast(node.child(1), compiler, block);
        parse_ast(node.child(3), compiler, block);
    } else {
        parse_ast(node.child(0), compiler, block);
        parse_ast(node.child(2), compiler, block);
    }
    let mut cloned_compiler = compiler.typestack.clone();
    let left_type = cloned_compiler.pop().unwrap();
    let right_type = cloned_compiler.pop().unwrap();

    let res = compiler.typestack.binop(binop.clone());
    if res.is_none() {
        compiler.add_error(errors::CompileTimeErrorType::BinopNotPossible { left: left_type, right: right_type }, node);
    }
    let (line, col_start, length) = util::get_node_position_and_span_unicode(node);
    block.binop(binop, TextSpan {
        line,
        col_start,
        length,
    });
}
