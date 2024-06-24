use std::fs;

use celsium::{
    block::Block,
    compiletime_helper::CompileTimeHelper,
    module::VISIBILITY,
    BuiltinTypes,
};
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{data_format, util};

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn wasm_print(s: &str);
    fn get_stumbrs_data() -> String;
}


pub fn stumbrs_define(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeHelper,
    is_wasm: bool
) {
    if title == "multiple_id_define" {
        if node.child(1).child(0).get_value().unwrap() != "STUMBRS" {
            panic!("Šeit var izmantot tikai 'STUMBRS' funkciju.");
        }
        let path = node.child(1).child(1).child(0).get_value().unwrap().to_owned();
        println!("{}", util::rem_first_and_last(&path));
        let data_file_content = if path == ""{
            get_stumbrs_data()
        }
        else{
            fs::read_to_string(util::rem_first_and_last(&path)).expect("Fails nav atrasts")
        };
        let res = data_format::parse_string(rem_first_and_last(&data_file_content).to_string());
        let reserrors = res.errors.clone().errors;
        if reserrors.len() > 0 {
            crate::parser_errors(reserrors, typestack);
        }
        let data_ast = res.get_ast();
        util::print_ast(data_ast.get_root());
    }
}
