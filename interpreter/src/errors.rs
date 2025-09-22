use std::process::exit;

use celsium::{ compiletime_helper::CompileTimeHelper, BuiltinTypes };
use colored::Colorize;
use hime_redist::{ast::AstNode, text::TextPosition};

use crate::util::{self, get_closest_node_location, str_from_data_type};

pub fn parser_error(unexpected: String, position: TextPosition, compilehelper: &mut CompileTimeHelper) {
    let error_title = if unexpected == "saraksts" {
        format!("Šajā vietā nepieļaujams simbols `{}`\nIespējams aizmirsi norādīt saraksta elementu datu tipu", unexpected)
    } else {
        format!("Šajā vietā nepieļaujams simbols `{}`", unexpected)
    };
    common_error(&error_title, position, compilehelper);
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
pub fn wrong_argument_type(func_name: String, arg_index: usize ,expected: &BuiltinTypes, found: &BuiltinTypes, node: AstNode, compilehelper: &mut CompileTimeHelper){
    let position = util::get_closest_node_location(node);
    let msg = format!("Fukcija `{}` kā {}. argumentu sagaida datu tipu `{}`, bet atrasts arguments ar tipu `{}`.", func_name, arg_index, str_from_data_type(expected), str_from_data_type(found));
    common_error(&msg, position, compilehelper);
}
pub fn notexistant_type(type_name: String, node: AstNode, compilehelper: &mut CompileTimeHelper){
    let position = util::get_closest_node_location(node);
    let msg = format!("Datu tips `{}` neeksistē. Ne kā vienkāršais tips, ne kā objekts.", type_name);
    common_error(&msg, position, compilehelper);
}
pub fn array_element_wrong_type(
    array_name: String,
    element_index: usize,
    expected_type: &BuiltinTypes,
    found_type: &BuiltinTypes,
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
    expected_type: &BuiltinTypes,
    found_type: &BuiltinTypes,
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

pub fn variable_not_indexable(
    data_type: &BuiltinTypes,
    compilehelper: &mut CompileTimeHelper, node: AstNode
) {
    common_error(
        &format!(
            "Datu tips `{}` Nav indeksējams",
            util::str_from_data_type(data_type)
        ),
        get_closest_node_location(node),
        compilehelper
    );
}

pub fn incorrect_variable_init_value(
    expected: &BuiltinTypes,
    found: &BuiltinTypes,
    compilehelper: &mut CompileTimeHelper, node: AstNode
) {
    common_error(
        &format!(
            "Nepareizs datu tips mainīgā sākotnējai vērtībai. Sagaidītais tips ir: `{}`, bet tika mēģināts piešķirt vērtību ar tipu: `{}`",
            util::str_from_data_type(expected),
            util::str_from_data_type(found)

        ),
        get_closest_node_location(node),
        compilehelper
    );
}

pub fn binop_not_possible(
    side1: &BuiltinTypes,
    side2: &BuiltinTypes,
    compilehelper: &mut CompileTimeHelper, node: AstNode
) {
    common_error(
        &format!(
            "Nav iespējams veikt šo darbību starp datu tipiem `{}` un `{}`",
            util::str_from_data_type(side1),
            util::str_from_data_type(side2)

        ),
        get_closest_node_location(node),
        compilehelper
    );
}

pub fn common_error(msg: &str, position: TextPosition, compilehelper: &mut CompileTimeHelper) {
    let path = &compilehelper.source_file_paths[compilehelper.current_file];
    println!("{}\n     {}\n     Faila \"{}\"\n     {}. rindiņā", "-----Kļūda: ".red(), msg.red(), path, position.line);
    exit(0);
}
