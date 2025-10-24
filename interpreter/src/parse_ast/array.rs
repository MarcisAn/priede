use celsium::{ block::Block, BuiltinTypes };
use hime_redist::ast::AstNode;

use crate::{
    errors::common_error,
    util::get_closest_node_location,
    Compiler,
};

use super::parse_ast;

pub fn array(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title == "array" {
        let mut element_types: Vec<BuiltinTypes> = vec![];
        for element in node.children() {
            parse_ast(element, compiler, block);
            let element_type = compiler.typestack.pop().unwrap();
            element_types.push(element_type);
        }
        let first_elem = &element_types.clone()[0];
        for element_type in element_types {
            if element_type != *first_elem {
                common_error(
                    "Visiem saraksta elementiem jābūt ar vienādiem datu tipiem.",
                    get_closest_node_location(node),
                    &mut compiler.helper
                );
            }
        }
        compiler.typestack.push(BuiltinTypes::Array { element_type: Box::new(first_elem.clone()), length: Some(node.children_count()) });
        block.create_array(node.children_count());
    }
}

// fn get_array_element(node: AstNode, compiler: &mut Compiler, array_name: &str, block: &mut Block) {
//     let array = util::get_closest_scope(
//         array_name.to_string(),
//         block.scope.clone(),
//         &mut compiler.helper,
//         node
//     );
//     if array.is_none() {
//         errors::undefined_var(
//             format!("Saraksts `{}` nav definēts", array_name),
//             &mut compiler.helper,
//             node
//         );
//     }
//     let array_type_anf_len = compiler.helper.get_array_type_and_length(array.unwrap()).unwrap();
//     let array_length = array_type_anf_len.1;
//     let array_type = array_type_anf_len.0;

//     //extra check if the index is number
//     if node.child(1).get_symbol().to_string() == "NUMBER" {
//         let index_number: usize = node.child(1).get_value().unwrap().parse().unwrap();
//         if array_length - 1 < index_number {
//             errors::array_element_index_too_high(
//                 array_name.to_string(),
//                 array_length,
//                 index_number,
//                 &mut compiler.helper,
//                 node
//             );
//         }
//     }

//     compiler.typestack.push(array_type);
//     block.load_from_array(array.unwrap());
// }
