use celsium::{block::{Block, TextSpan}, bytecode::BINOP};
use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};

use crate::{errors, util, util::lookup_variable, Compiler};

use super::parse_ast;

fn span_of(node: AstNode) -> TextSpan {
    let (line, col_start, length) = util::get_node_position_and_span_unicode(node);
    TextSpan { line, col_start, length }
}

fn lookup_var_id_or_error(
    varname: String,
    err_node: AstNode,
    compiler: &mut Compiler,
    block: &Block,
) -> Option<usize> {
    let var_id = lookup_variable(varname.clone(), block.scope.clone(), &mut compiler.helper, err_node);
    if var_id.is_none() {
        compiler.add_error(
            errors::CompileTimeErrorType::VariableNotDefined { varname },
            err_node,
        );
    }
    var_id
}

pub fn id_assign(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title == "id_assign" {
        let operator = node.child(1).get_value().unwrap();

        // For ++/-- the operator token is at child(1); otherwise we want the RHS token span.
        let op_span = if operator == "++" || operator == "--" {
            span_of(node.child(1))
        } else {
            span_of(node.child(2))
        };

        // Parse LHS (or type annotation RHS for ":") first.
        if operator == ":" {
            parse_ast(node.child(2), compiler, block);
        } else {
            parse_ast(node.child(0), compiler, block);
            // Preserve existing behavior: type-check LHS but keep typestack unchanged.
            let data_type = compiler.typestack.pop().unwrap();
            compiler.typestack.push(data_type);
        }

        // Parse RHS for all assignment operators that take a value.
        if operator != "--" && operator != "++" && operator != ":" {
            parse_ast(node.child(2), compiler, block);
        }

        match operator {
            "+:" => {
                compiler.typestack.binop(BINOP::Add);
                block.binop(BINOP::Add, op_span);
            }
            "-:" => {
                compiler.typestack.binop(BINOP::Subtract);
                block.binop(BINOP::Subtract, op_span);
            }
            "*:" => {
                compiler.typestack.binop(BINOP::Multiply);
                block.binop(BINOP::Multiply, op_span);
            }
            "/:" => {
                compiler.typestack.binop(BINOP::Divide);
                block.binop(BINOP::Divide, op_span);
            }
            "++" => {
                block.load_int(1);
                compiler.typestack.binop(BINOP::Add);
                block.binop(BINOP::Add, op_span);
            }
            "--" => {
                block.load_int(1);
                compiler.typestack.binop(BINOP::Subtract);
                block.binop(BINOP::Subtract, op_span);
            }
            ":" => {
                // Type annotation; parsing done above.
            }
            _ => {}
        }

        let assignable = node.child(0).get_symbol().name;
        if assignable == "ID" {
            let var_name = node.child(0).get_value().unwrap().to_string();
            let Some(var_id) = lookup_var_id_or_error(var_name, node, compiler, block) else {
                return;
            };
            block.assign_variable(var_id);
        } else if assignable == "dot_call" {
            let object_var_name = node.child(0).child(0).get_value().unwrap().to_string();
            let Some(var_id) = lookup_var_id_or_error(object_var_name, node, compiler, block) else {
                return;
            };
            let field_name = node.child(0).child(1).get_value().unwrap().to_string();
            block.set_object_field(var_id, field_name);
        }
    } else if title == "array_assign" {
        let var_name = node.child(0).child(0).get_value().unwrap().to_string();
        let Some(var_id) = lookup_var_id_or_error(var_name, node, compiler, block) else {
            return;
        };

        parse_ast(node.child(1), compiler, block);
        parse_ast(node.child(0).child(1), compiler, block);
        block.assign_to_array(var_id);
    }
}
