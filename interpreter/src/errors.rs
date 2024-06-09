use std::process::exit;

use annotate_snippets::{ Level, Renderer, Snippet };
use anstream;
use celsium::{ compiletime_helper::CompileTimeHelper, BUILTIN_TYPES };
use colored::Colorize;
use hime_redist::{ast::AstNode, text::TextPosition};

use crate::util::{self, get_closest_node_location};

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
pub fn undefined_var(msg: String, compilehelper: &mut CompileTimeHelper, node: AstNode) {
    common_error(&msg, util::get_closest_node_location(node), compilehelper);
}
pub fn undefined_func(msg: String, compilehelper: &mut CompileTimeHelper, node: AstNode) {
    common_error(&msg, util::get_closest_node_location(node), compilehelper);
}
pub fn array_element_wrong_type(
    array_name: String,
    element_index: usize,
    expected_type: BUILTIN_TYPES,
    found_type: BUILTIN_TYPES,
    compilehelper: &mut CompileTimeHelper, node: AstNode
) {
    let expected = match expected_type {
        BUILTIN_TYPES::MAGIC_INT => "skaitlis",
        BUILTIN_TYPES::BOOL => "būls",
        BUILTIN_TYPES::STRING => "teksts",
        BUILTIN_TYPES::OBJECT => "objekts",
        BUILTIN_TYPES::FLOAT => "decimālskaitlis",
    };
    let found = match found_type {
        BUILTIN_TYPES::MAGIC_INT => "skaitlis",
        BUILTIN_TYPES::BOOL => "būls",
        BUILTIN_TYPES::STRING => "teksts",
        BUILTIN_TYPES::OBJECT => "objekts",
        BUILTIN_TYPES::FLOAT => "decimālskaitlis",
    };
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
    let expected = match expected_type {
        BUILTIN_TYPES::MAGIC_INT => "skaitlis",
        BUILTIN_TYPES::BOOL => "būls",
        BUILTIN_TYPES::STRING => "teksts",
        BUILTIN_TYPES::OBJECT => "objekts",
        BUILTIN_TYPES::FLOAT => "decimālskaitlis",
    };
    let found = match found_type {
        BUILTIN_TYPES::MAGIC_INT => "skaitlis",
        BUILTIN_TYPES::BOOL => "būls",
        BUILTIN_TYPES::STRING => "teksts",
        BUILTIN_TYPES::OBJECT => "objekts",
        BUILTIN_TYPES::FLOAT => "decimālskaitlis",
    };
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
