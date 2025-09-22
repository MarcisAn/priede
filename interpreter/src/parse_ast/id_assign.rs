use celsium::{ block::{ self, Block }, bytecode::BINOP, compiletime_helper::CompileTimeHelper };
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
        } else{
            var_id_test.unwrap()
        };
        let data_type = compiler.helper.get_var_type(var_id).unwrap();
        compiler.typestack.push(data_type.clone());
        block.load_variable(var_id);

        if operator == ":" {
            parse_ast(node.child(2), compiler, block);
        } else if operator == "+:" {
            block.load_variable(var_id);
            let data_type = compiler.helper.get_var_type(var_id).unwrap();
            compiler.typestack.push(data_type);
            parse_ast(node.child(2), compiler, block);
            compiler.typestack.binop(BINOP::Add);
            block.binop(BINOP::Add);
        } else if operator == "-:" {
            block.load_variable(var_id);
            let data_type = compiler.helper.get_var_type(var_id).unwrap();
            compiler.typestack.push(data_type);
            parse_ast(node.child(2), compiler, block);
            block.binop(BINOP::Subtract);
            compiler.typestack.binop(BINOP::Subtract);
        } else if operator == "*:" {
            block.load_variable(var_id);
            let data_type = compiler.helper.get_var_type(var_id).unwrap();
            compiler.typestack.push(data_type);
            parse_ast(node.child(2), compiler, block);
            block.binop(BINOP::Multiply);
            compiler.typestack.binop(BINOP::Multiply);
        } else if operator == "/:" {
            block.load_variable(var_id);
            let data_type = compiler.helper.get_var_type(var_id).unwrap();
            compiler.typestack.push(data_type);
            parse_ast(node.child(2), compiler, block);
            block.binop(BINOP::Divide);
        } else if operator == "++" {
            block.load_variable(var_id);
            let data_type = compiler.helper.get_var_type(var_id).unwrap();
            compiler.typestack.push(data_type);
            block.load_int(1);
            compiler.typestack.binop(BINOP::Add);
            block.binop(BINOP::Add);
        } else if operator == "--" {
            block.load_variable(var_id);
            let data_type = compiler.helper.get_var_type(var_id).unwrap();
            compiler.typestack.push(data_type);
            block.load_int(1);
            block.binop(BINOP::Subtract);
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
