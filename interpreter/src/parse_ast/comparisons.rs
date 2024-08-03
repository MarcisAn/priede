use std::process::exit;

use celsium::bytecode::BINOP;
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::Compiler;

use super::parse_ast;

pub fn comparisons(node: AstNode, title: &str, compiler: &mut Compiler) {
    let target_reg = compiler.register_counter;
    if title == "comp_s" {
        let sign = node.child(1).get_value().unwrap();
        parse_ast(node.child(0), compiler);
        let reg_a = compiler.helper.get_top().unwrap().register_id;
        parse_ast(node.child(2), compiler);
        let reg_b = compiler.helper.get_top().unwrap().register_id;

        let binop = match sign {
            "=" => BINOP::Eq,
            ">" => BINOP::LargerThan,
            ">=" => BINOP::LargerOrEq,
            "<" => BINOP::LessThan,
            "<=" => BINOP::LessOrEq,
            "!=" => BINOP::NotEq,
            _ => panic!("Neatpazīts salīdzinājuma simbols"),
        };

        let checked_type = compiler.helper.binop(binop, target_reg);
        if checked_type.is_none() {
            compiler.add_error(
                crate::compiler::CompileErrorType::MathTypes,
                node.child(1).get_position().unwrap().line,
                node.child(1).get_position().unwrap().column,
                node.child(1).get_span().unwrap().length
            );
            exit(0);
        }
        let binop = match sign {
            "=" => BINOP::Eq,
            ">" => BINOP::LargerThan,
            ">=" => BINOP::LargerOrEq,
            "<" => BINOP::LessThan,
            "<=" => BINOP::LessOrEq,
            "!=" => BINOP::NotEq,
            _ => panic!("Neatpazīts salīdzinājuma simbols"),
        };
        compiler.block.binop(BINOP::Eq, target_reg, reg_a, reg_b);
        compiler.register_counter += 1;

    }

    if ["un", "vai", "xvai"].contains(&title) {
        parse_ast(node.child(0), compiler);
        let a_reg = compiler.helper.get_top().unwrap().register_id;
        parse_ast(node.child(1), compiler);
        let b_reg = compiler.helper.get_top().unwrap().register_id;

        if title == "un" {
            compiler.helper.binop(BINOP::And, target_reg);
            compiler.block.binop(BINOP::And, a_reg, b_reg, target_reg);
        } else if title == "vai" {
            parse_ast(node.child(0), compiler);
            parse_ast(node.child(1), compiler);
            compiler.helper.binop(BINOP::Or, target_reg);
            compiler.block.binop(BINOP::Or, a_reg, b_reg, target_reg);
        } else if title == "xvai" {
            parse_ast(node.child(0), compiler);
            parse_ast(node.child(1), compiler);
            compiler.helper.binop(BINOP::Xor, target_reg);
            compiler.block.binop(BINOP::Xor, a_reg, b_reg, target_reg);
        }
        compiler.register_counter += 1;
    }

}
