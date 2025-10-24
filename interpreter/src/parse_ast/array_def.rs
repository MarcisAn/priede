use celsium::{ block::Block, BuiltinTypes };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };
use crate::{ errors::incorrect_variable_init_value, util, Compiler };

use super::parse_ast;

fn get_nested_array_type(node: AstNode, compiler: &mut Compiler) -> BuiltinTypes {
    if node.get_symbol().name == "ID" {
        let data_type_str = node.get_value().unwrap();
        return util::get_data_type_from_id(&mut compiler.helper, data_type_str, node);
    } else {
        return BuiltinTypes::Array {
            element_type: Box::new(get_nested_array_type(node.child(0), compiler)),
            length: None,
        };
    }
}

pub fn array_def(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title == "array_def" {
        let is_exported = false;

        let data_type_marked = get_nested_array_type(
            node.child(0 + (is_exported as usize)),
            compiler
        );

        let varname = node
            .child(1 + (is_exported as usize))
            .get_value()
            .unwrap()
            .to_string();

        //parse the init value
        parse_ast(node.child(2 + (is_exported as usize)), compiler, block);
        let typ_of_init_value = compiler.typestack.pop().unwrap();

        let init_value_to_compare = match typ_of_init_value.clone() {
            BuiltinTypes::Array { element_type, length: _ } =>
                BuiltinTypes::Array { element_type: element_type, length: None },
            _ => panic!(),
        };

        let marked_value_to_compare = match data_type_marked.clone() {
            BuiltinTypes::Array { element_type, length: _ } =>
                BuiltinTypes::Array { element_type: element_type, length: None },
            _ => panic!(),
        };

        if init_value_to_compare != marked_value_to_compare {
            incorrect_variable_init_value(
                &data_type_marked,
                &typ_of_init_value,
                &mut compiler.helper,
                node
            );
            return;
        }

        let var_id = compiler.helper.def_var(
            varname.clone(),
            data_type_marked.clone(),
            block.scope.clone(),
            is_exported
        );
        block.define_variable(var_id.unwrap());
    }
}
