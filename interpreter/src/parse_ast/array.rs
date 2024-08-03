use celsium::BuiltinTypes;
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ errors, util, Compiler };

use super::parse_ast;

pub fn array(node: AstNode, title: &str, compiler: &mut Compiler) {
    if title == "array" {
        let array_name = node.child(0).get_value().unwrap();
        let source_id = util::get_closest_scope(
            array_name.to_string(),
            compiler.block.scope.clone(),
            &mut compiler.helper,
            node
        );
        let data_type = compiler.helper.get_var_type(source_id.unwrap()).unwrap();

        //parse the index
        parse_ast(node.child(1), compiler);
        let index_type = compiler.helper.pop().unwrap();
        if index_type.data_type != BuiltinTypes::MagicInt {
            errors::array_element_wrong_type_index(
                array_name.to_string(),
                BuiltinTypes::MagicInt,
                index_type.data_type,
                &mut compiler.helper,
                node
            );
        }

        match data_type {
            BuiltinTypes::Array { element_type } => get_array_element(node, compiler, array_name),
            _ => errors::variable_not_indexable(data_type, &mut compiler.helper, node),
        }
    }
}

fn get_array_element(node: AstNode, compiler: &mut Compiler, array_name: &str) {
    let array = util::get_closest_scope(
        array_name.to_string(),
        compiler.block.scope.clone(),
        &mut compiler.helper,
        node
    );
    if array.is_none() {
        errors::undefined_var(
            format!("Saraksts `{}` nav definēts", array_name),
            &mut compiler.helper,
            node
        );
    }
    let array_type_anf_len = compiler.helper.get_array_type_and_length(array.unwrap()).unwrap();
    let array_length = array_type_anf_len.1;
    let array_type = array_type_anf_len.0;

    //extra check if the index is number
    if node.child(1).get_symbol().to_string() == "NUMBER" {
        let index_number: usize = node.child(1).get_value().unwrap().parse().unwrap();
        if array_length - 1 < index_number {
            errors::array_element_index_too_high(
                array_name.to_string(),
                array_length,
                index_number,
                &mut compiler.helper,
                node
            );
        }
    }

    compiler.helper.push(array_type, compiler.register_counter);
    compiler.block.load_from_array(array.unwrap(), compiler.register_counter);
    compiler.register_counter += 1;
}
