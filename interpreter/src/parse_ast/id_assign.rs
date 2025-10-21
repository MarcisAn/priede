use celsium::{
    block::{ self, Block, TextSpan },
    bytecode::BINOP,
    compiletime_helper::CompileTimeHelper,
};
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ errors, util::get_closest_scope, Compiler };

use super::parse_ast;

pub fn id_assign(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title == "id_assign" {

        let operator = node.child(1).get_value().unwrap();
        let var_name = node.child(0).get_value().unwrap();
        let var_id_test = get_closest_scope(
            var_name.to_string(),
            block.scope.clone(),
            &mut compiler.helper,
            node
        );

        let var_id: usize = if var_id_test.is_none() {
            errors::undefined_var(
                format!(
                    "Mainīgais ar nosaukumu '{}' nav definēts šajā blokā.",
                    node.child(0).get_value().unwrap()
                ),
                &mut compiler.helper,
                node
            );
            return;
        } else {
            var_id_test.unwrap()
        };


        let data_type = compiler.helper.get_var_type(var_id).unwrap();
        compiler.typestack.push(data_type.clone());

        let node_span = node.child(2).get_total_position_and_span().unwrap();

        if operator == ":" {
            parse_ast(node.child(2), compiler, block);
        } else {
            block.load_variable(var_id, block::TextSpan {
                line: node_span.0.line,
                col_start: node_span.0.column,
                length: node_span.1.length,
            });

            let data_type = compiler.helper.get_var_type(var_id).unwrap();
            compiler.typestack.push(data_type);
            parse_ast(node.child(2), compiler, block);
        }
        if operator == "+:" {
            compiler.typestack.binop(BINOP::Add);
            block.binop(BINOP::Add, TextSpan {
                line: node_span.0.line,
                col_start: node_span.0.column,
                length: node_span.1.length,
            });
        } else if operator == "-:" {
            compiler.typestack.binop(BINOP::Subtract);
            block.binop(BINOP::Subtract, TextSpan {
                line: node_span.0.line,
                col_start: node_span.0.column,
                length: node_span.1.length,
            });
        } else if operator == "*:" {
            compiler.typestack.binop(BINOP::Multiply);
            block.binop(BINOP::Multiply, TextSpan {
                line: node_span.0.line,
                col_start: node_span.0.column,
                length: node_span.1.length,
            });
        } else if operator == "/:" {
            compiler.typestack.binop(BINOP::Divide);
            block.binop(BINOP::Divide, TextSpan {
                line: node_span.0.line,
                col_start: node_span.0.column,
                length: node_span.1.length,
            });
        } else if operator == "++" {
            compiler.typestack.binop(BINOP::Add);
            block.binop(BINOP::Add, TextSpan {
                line: node_span.0.line,
                col_start: node_span.0.column,
                length: node_span.1.length,
            });
        } else if operator == "--" {
            block.load_int(1);
            block.binop(BINOP::Subtract, TextSpan {
                line: node_span.0.line,
                col_start: node_span.0.column,
                length: node_span.1.length,
            });
        }
        block.assign_variable(var_id);
    } else if title == "array_assign" {
        let var_name = node.child(0).child(0).get_value().unwrap();
        let var_id = get_closest_scope(
            var_name.to_string(),
            block.scope.clone(),
            &mut compiler.helper,
            node
        ).unwrap();

        parse_ast(node.child(1), compiler, block);
        parse_ast(node.child(0).child(1), compiler, block);
        block.assign_to_array(var_id);
    }
}
