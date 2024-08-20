use std::process::exit;

use crate::Compiler;
use celsium::bytecode::BINOP;
use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};

use super::parse_ast;

pub fn math_ops(
    node: AstNode,
    title: &str,
    compiler: &mut Compiler
) {
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
            compiler
        );
    }
}
fn calculate(
    binop: BINOP,
    node: AstNode,
    compiler: &mut Compiler
) {
    parse_ast(node.child(0), compiler);
    parse_ast(node.child(2), compiler);
    let res = compiler.helper.binop(binop.clone());
    if res.is_none() {
        compiler.add_error(
                crate::compiler::CompileErrorType::MathTypes,
                node.child(1).get_position().unwrap().line,
                node.child(1).get_position().unwrap().column,
                node.child(1).get_span().unwrap().length
            );
        exit(0);
    }
    compiler.block.binop(binop);
}
