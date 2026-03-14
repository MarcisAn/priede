use celsium::{ BuiltinTypes, block::{ Block, TextSpan }, bytecode::BINOP };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, errors, util::{ self, are_types_equal, lookup_variable } };

use super::parse_ast;

fn span_of(node: AstNode) -> TextSpan {
    let (line, col_start, length) = util::get_node_position_and_span_unicode(node);
    TextSpan { line, col_start, length }
}

pub fn lookup_var_id_or_error(
    varname: String,
    err_node: AstNode,
    compiler: &mut Compiler,
    block: &Block
) -> Option<usize> {
    let var_id = lookup_variable(
        varname.clone(),
        block.scope.clone(),
        &mut compiler.helper,
        err_node
    );
    if var_id.is_none() {
        compiler.add_error(errors::CompileTimeErrorType::VariableNotDefined { varname }, err_node);
    }
    var_id
}

fn handle_assignment(
    lhs_node: AstNode,
    rhs_node: Option<AstNode>,
    operator: &str,
    operator_span: TextSpan,
    err_node: AstNode,
    compiler: &mut Compiler,
    block: &mut Block
) {
    let assignable = lhs_node.get_symbol().name;
    
    // Handle array assignment (indexable)
    if assignable == "indexable" {
        let var_name = lhs_node.child(0).get_value().unwrap().to_string();
        let Some(var_id) = lookup_var_id_or_error(var_name, err_node, compiler, block) else {
            return;
        };

        // Parse RHS value
        if let Some(rhs) = rhs_node {
            parse_ast(rhs, compiler, block);
        }
        let type_to_assign = compiler.typestack.pop().unwrap();
        let type_of_array = compiler.get_var_type_from_id(var_id).unwrap();
        
        // Validate array type
        let _ = match type_of_array {
            BuiltinTypes::String => todo!("string assign"),
            BuiltinTypes::Array { element_type: _, length: _ } => print!(""),
            _ => {
                compiler.add_error(
                    errors::CompileTimeErrorType::ValueNotIndexable {
                        found_type: type_of_array.clone(),
                    },
                    err_node
                );
                return;
            }
        };
        
        // Check type compatibility
        if !are_types_equal(
            &type_of_array,
            &(BuiltinTypes::Array {
                element_type: Box::new(type_to_assign.clone()),
                length: None,
            })
        ) {
            compiler.add_error(
                errors::CompileTimeErrorType::WrongArrayAssignValue {
                    array_type: type_of_array,
                    assigned_type: type_to_assign,
                },
                err_node
            );
        }
        
        // Parse index expression
        parse_ast(lhs_node.child(1), compiler, block);
        block.assign_to_array(var_id);
        return;
    }

    // Handle regular assignments (ID, dot_call, etc.)
    // Parse LHS (or type annotation RHS for ":") first.
    if operator == ":" {
        if let Some(rhs) = rhs_node {
            parse_ast(rhs, compiler, block);
        }
    } else {
        parse_ast(lhs_node, compiler, block);
        // Preserve existing behavior: type-check LHS but keep typestack unchanged.
        let data_type = compiler.typestack.pop().unwrap();
        compiler.typestack.push(data_type);
    }

    // Parse RHS for all assignment operators that take a value.
    if operator != "--" && operator != "++" && operator != ":" {
        if let Some(rhs) = rhs_node {
            parse_ast(rhs, compiler, block);
        }
    }

    match operator {
        "+:" => {
            compiler.typestack.binop(BINOP::Add);
            block.binop(BINOP::Add, lhs_node.id());
        }
        "-:" => {
            compiler.typestack.binop(BINOP::Subtract);
            block.binop(BINOP::Subtract, lhs_node.id());
        }
        "*:" => {
            compiler.typestack.binop(BINOP::Multiply);
            block.binop(BINOP::Multiply, lhs_node.id());
        }
        "/:" => {
            compiler.typestack.binop(BINOP::Divide);
            block.binop(BINOP::Divide, lhs_node.id());
        }
        "++" => {
            block.load_int(1);
            compiler.typestack.binop(BINOP::Add);
            block.binop(BINOP::Add, lhs_node.id());
        }
        "--" => {
            block.load_int(1);
            compiler.typestack.binop(BINOP::Subtract);
            block.binop(BINOP::Subtract, lhs_node.id());
        }
        ":" => {
            // Type annotation; parsing done above.
        }
        _ => {}
    }

    if assignable == "ID" {
        let var_name = lhs_node.get_value().unwrap().to_string();
        let Some(var_id) = lookup_var_id_or_error(var_name, err_node, compiler, block) else {
            return;
        };
        block.assign_variable(var_id);
    } else if assignable == "dot_call" {
        let object_var_name = lhs_node.child(0).get_value().unwrap().to_string();
        let Some(var_id) = lookup_var_id_or_error(object_var_name, err_node, compiler, block) else {
            return;
        };
        let field_name = lhs_node.child(1).get_value().unwrap().to_string();
        block.set_object_field(var_id, field_name);
    }
}

pub fn id_assign(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title == "id_assign" {
        let lhs_node = node.child(0);
        let operator_node = node.child(1);
        let operator = operator_node.get_value().unwrap();

        // For ++/-- the operator token is at child(1); otherwise we want the RHS token span.
        let op_span = if operator == "++" || operator == "--" {
            span_of(operator_node)
        } else {
            span_of(node.child(2))
        };

        let rhs_node = if operator == "++" || operator == "--" {
            None
        } else {
            Some(node.child(2))
        };

        handle_assignment(lhs_node, rhs_node, operator, op_span, node, compiler, block);
    } else if title == "array_assign" {
        let lhs_node = node.child(0); // indexable node
        let rhs_node = Some(node.child(1)); // value to assign
        let operator = "="; // Simple assignment for arrays
        let op_span = span_of(node.child(1)); // Use RHS span for operator

        handle_assignment(lhs_node, rhs_node, operator, op_span, node, compiler, block);
    }
}
