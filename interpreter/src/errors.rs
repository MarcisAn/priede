use std::process::exit;

use annotate_snippets::{ Level, Renderer, Snippet };
use anstream;
use celsium::{ compiletime_helper::CompileTimeHelper, BUILTIN_TYPES };
use colored::Colorize;
use hime_redist::{ast::AstNode, text::TextPosition};

use crate::util::{self, get_closest_node_location, str_from_data_type};

pub fn parser_error(unexpected: String, position: TextPosition, compilehelper: &mut CompileTimeHelper) {
    let error_title = if unexpected == "saraksts" {
        format!("NEATPAZĪTS SIMBOLS `{}`\nIespējams aizmirsi norādīt saraksta elementu datu tipu", unexpected)
    } else {
        format!("NEATPAZĪTS SIMBOLS `{}`", unexpected)
    };
    common_error(&error_title, position, compilehelper);
}
pub fn math_error(compilehelper: &mut CompileTimeHelper, node: AstNode) {
    common_error(
        "Ar šiem datu tipiem nevar veikt šo matemātisko darbību",
        util::get_closest_node_location(node),
        compilehelper
    );
}
pub fn incorect_init_value(msg: String, compilehelper: &mut CompileTimeHelper, node: AstNode) {
    common_error(&msg, util::get_closest_node_location(node), compilehelper);
}
pub fn already_defined(msg: String, compilehelper: &mut CompileTimeHelper, node: AstNode) {
    common_error(&msg, util::get_closest_node_location(node), compilehelper);
}
pub fn undefined_var(msg: String, compilehelper: &mut CompileTimeHelper, node: AstNode) {
    common_error(&msg, util::get_closest_node_location(node), compilehelper);
}
pub fn undefined_func(msg: String, compilehelper: &mut CompileTimeHelper, node: AstNode) {
    common_error(&msg, util::get_closest_node_location(node), compilehelper);
}
pub fn wrong_argument_count(func_name: String, expected: usize, found: usize, node: AstNode, compilehelper: &mut CompileTimeHelper){
    let position = util::get_closest_node_location(node);
    let msg = format!("Fukcija {} sagaida {} argumentu{}, bet šeit tiek padot{} {} argument{}", func_name, expected, if expected == 1 {""} else {"s"}, if expected == 1 {"s"} else {"i"}, found, if expected == 1 {"s"} else {"i"});
    common_error(&msg, position, compilehelper);
}
pub fn wrong_argument_type(func_name: String, arg_index: usize ,expected: BUILTIN_TYPES, found: BUILTIN_TYPES, node: AstNode, compilehelper: &mut CompileTimeHelper){
    let position = util::get_closest_node_location(node);
    let msg = format!("Fukcija `{}` kā {}. argumentu sagaida datu tipu `{}`, bet atrasts arguments ar tipu `{}`.", func_name, arg_index, str_from_data_type(expected), str_from_data_type(found));
    common_error(&msg, position, compilehelper);
}
pub fn array_element_wrong_type(
    array_name: String,
    element_index: usize,
    expected_type: BUILTIN_TYPES,
    found_type: BUILTIN_TYPES,
    compilehelper: &mut CompileTimeHelper, node: AstNode
) {
    let expected = str_from_data_type(expected_type);
    let found = util::str_from_data_type(found_type);
    common_error(
        &format!(
            "Definējot sarakstu `{}`, tā sākotnējā vērtība pozīcijā {} ir ar nepareizu datu tipu. Nepieciešams {}, bet atrasts {}.",
            array_name,
            element_index,
            expected,
            found
        ),
        util::get_closest_node_location(node),
        compilehelper
    );
}
pub fn array_element_wrong_type_index(
    array_name: String,
    expected_type: BUILTIN_TYPES,
    found_type: BUILTIN_TYPES,
    compilehelper: &mut CompileTimeHelper, node: AstNode
) {
    let expected = str_from_data_type(expected_type);
    let found = util::str_from_data_type(found_type);
    common_error(
        &format!(
            "Nepareizs indeksa datu tips, indeksējot sarakstu `{}`. Nepieciešams {}, bet atrasts {}.",
            array_name,
            expected,
            found
        ),
        get_closest_node_location(node),
        compilehelper
    );
}
pub fn array_element_index_too_high(
    array_name: String,
    expected_index: usize,
    found_index: usize,
    compilehelper: &mut CompileTimeHelper, node: AstNode
) {
    common_error(
        &format!(
            "Sarakstu `{}` garums ir {}, bet ir mēģinājums to indeksēt ar indeksu {}.\nSarakstu indeksi tiek skaitīti no nulles.",
            array_name,
            expected_index,
            found_index
        ),
        get_closest_node_location(node),
        compilehelper
    );
}

fn common_error(msg: &str, position: TextPosition, compilehelper: &mut CompileTimeHelper) {
    let path = &compilehelper.source_file_paths[compilehelper.current_file];
    println!("{}\n{}\nFaila \"{}\"\n{}. rindiņā", "Kļūda: ".red(), msg.red(), path, position.line);
    exit(0);
}
