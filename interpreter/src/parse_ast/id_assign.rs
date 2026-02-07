use celsium::{ block::{ self, Block, TextSpan }, bytecode::BINOP };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ errors, util::get_closest_scope, Compiler };

use super::parse_ast;

pub fn id_assign(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title == "id_assign" {
        let operator = node.child(1).get_value().unwrap();
        // let var_name = node.child(0).get_value().unwrap();
        // let var_id_test = get_closest_scope(
        //     var_name.to_string(),
        //     block.scope.clone(),
        //     &mut compiler.helper,
        //     node
        // );

        // let var_id: usize = if var_id_test.is_none() {
        //     // println!("erored here");
        //     compiler.add_error(
        //         errors::CompileTimeErrorType::VariableNotDefined {
        //             varname: node.child(0).get_value().unwrap().to_string(),
        //         },
        //         node
        //     );
        //     return;
        // } else {
        //     var_id_test.unwrap()
        // };

        let node_span = if operator == "++" || operator == "--" {
            node.child(1).get_total_position_and_span().unwrap()
        } else {
            node.child(2).get_total_position_and_span().unwrap()
        };
        if operator == ":" {
            parse_ast(node.child(2), compiler, block);
        } else {
            parse_ast(node.child(0), compiler, block);
            let data_type = compiler.typestack.pop().unwrap();
            compiler.typestack.push(data_type);
        }
        if operator != "--" && operator != "++" && operator != ":" {
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
            block.load_int(1);
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
        let assignable = node.child(0).get_symbol().name;
        if assignable == "ID" {
            let var_name = node.child(0).get_value().unwrap();
            let var_id_test = get_closest_scope(
                var_name.to_string(),
                block.scope.clone(),
                &mut compiler.helper,
                node
            );

            let var_id: usize = if var_id_test.is_none() {
                // println!("erored here");
                compiler.add_error(
                    errors::CompileTimeErrorType::VariableNotDefined {
                        varname: node.child(0).get_value().unwrap().to_string(),
                    },
                    node
                );
                return;
            } else {
                var_id_test.unwrap()
            };
            block.assign_variable(var_id);
        } else if assignable == "dot_call"{
            let object_var_name = node.child(0).child(0).get_value().unwrap();
            let var_id_test = get_closest_scope(
                object_var_name.to_string(),
                block.scope.clone(),
                &mut compiler.helper,
                node
            );

            let var_id: usize = if var_id_test.is_none() {
                // println!("erored here");
                compiler.add_error(
                    errors::CompileTimeErrorType::VariableNotDefined {
                        varname: node.child(0).get_value().unwrap().to_string(),
                    },
                    node
                );
                return;
            } else {
                var_id_test.unwrap()
            };
            block.set_object_field(var_id, node.child(0).child(1).get_value().unwrap().to_string());
        }
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
