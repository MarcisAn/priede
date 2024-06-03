use celsium::{ block::Block, compile_time_checker::CompileTimeChecker, BUILTIN_TYPES };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::errors;

use super::parse_ast;

pub fn array(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeChecker,
    is_wasm: bool
) {
    if title == "array" {
        let array_name = node.child(0).get_value().unwrap();
        //parse the index
        parse_ast(node.child(1), block, is_wasm, typestack);
        let index_type = typestack.pop().unwrap();
        if index_type != BUILTIN_TYPES::MAGIC_INT {
            errors::array_element_wrong_type_index(array_name.to_string(), BUILTIN_TYPES::MAGIC_INT, index_type, &typestack.source_files[typestack.current_file],
                    &typestack.source_file_paths[typestack.current_file],
                    node.child(1).get_position().unwrap().line,
                    node.child(1).get_position().unwrap().column);
        }
        let check = typestack.check_array_type_and_length(array_name);
        if check.is_none(){
            errors::undefined_var(format!("Saraksts `{}` nav definēts", array_name), &typestack.source_files[typestack.current_file],
                    &typestack.source_file_paths[typestack.current_file],
                    node.child(1).get_position().unwrap().line,
                    node.child(1).get_position().unwrap().column);
        }
        let (array_type, array_length) = check.unwrap();
        if node.child(1).get_symbol().to_string() == "NUMBER"{
            let index_number: usize = node.child(1).get_value().unwrap().parse().unwrap();
            if array_length-1 < index_number{
                 errors::array_element_index_too_high(array_name.to_string(), array_length, index_number, &typestack.source_files[typestack.current_file],
                    &typestack.source_file_paths[typestack.current_file],
                    node.child(1).get_position().unwrap().line);
            }
        }
        //TODO:arrays
        //typestack.push(array_type);
        //load using the index at top of the stack
        //block.load_from_array(array_name);
    }
}
