use celsium::block::{Block, TextSpan};
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

mod id_assign;
use id_assign::id_assign;
mod math_ops;
use math_ops::math_ops;
mod var_def;
use var_def::var_def;
mod constants;
use constants::parse_constants;
mod comparisons;
use comparisons::comparisons;
mod array;
use array::array;
mod func_call;
use func_call::func_call;
mod func_def;
use func_def::func_def;
mod loops;
use loops::loops;
mod if_stat;
use if_stat::if_stat;
mod id;
use id::id;
mod include;
use include::include;
mod array_def;
use array_def::array_def;
mod index;
use index::index;
mod objects;
use objects::objects;
mod dot_syntax;
use dot_syntax::dot_syntax;

use crate::{ Compiler, util };

pub fn parse_ast(node: AstNode, compiler: &mut Compiler, block: &mut Block) {
    let title = node.get_symbol().to_string();

    if title == "block" {
        for i in node.children() {
            parse_ast(i, compiler, block);
        }
    }

    

    if title == "exp_atom" {
        parse_ast(node.child(1), compiler, block);
        let (line, col_start, length) = util::get_node_position_and_span_unicode(node);

        block.binop(celsium::bytecode::BINOP::Not, TextSpan {
            line,
            col_start,
            length,
        });
    }
    if title == "BREAK" {
        let (line, col_start, length) = util::get_node_position_and_span_unicode(node);
        block.break_loop(TextSpan { line, col_start, length } );
    }
    if title == "CONTINUE" {
        let (line, col_start, length) = util::get_node_position_and_span_unicode(node);
        block.continue_loop(TextSpan { line, col_start, length });
    }
    index(node, &title, compiler, block);
    id(node, &title, compiler, block);
    if_stat(node, &title, compiler, block);
    loops(node, &title, compiler, block);
    func_def(node, &title, compiler, block);
    func_call(node, &title, compiler, block);
    array(node, &title, compiler, block);
    array_def(node, &title, compiler, block);
    math_ops(node, &title, compiler, block);
    comparisons(node, &title, compiler, block);
    id_assign(node, &title, compiler, block);
    parse_constants(node, &title, compiler, block);
    var_def(node, &title, compiler, block);
    include(node, &title, compiler, block);
    objects(node, &title, compiler, block);
    dot_syntax(node, &title, compiler, block);


}
