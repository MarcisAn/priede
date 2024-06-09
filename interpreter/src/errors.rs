use std::process::exit;

use annotate_snippets::{ Level, Renderer, Snippet };
use anstream;
use celsium::BUILTIN_TYPES;
use colored::Colorize;
use hime_redist::text::TextPosition;

pub fn parser_error(unexpected: String, source: &str, path: &str, line: usize, col: usize) {
    let error_title = if unexpected == "saraksts" {
        format!("NEATPAZĪTS SIMBOLS `{}`\nIespējams aizmirsi norādīt saraksta elementu datu tipu", unexpected)
    } else {
        format!("NEATPAZĪTS SIMBOLS `{}`", unexpected)
    };
    common_error(error_title.to_string(), line, path);
}
pub fn math_error(msg: &str, source: &str, path: &str, position: TextPosition) {
    let error_title = &format!("{}", msg);
    common_error(msg.to_string(), position.line, path);
}
pub fn incorect_init_value(msg: String, source: &str, path: &str, position: TextPosition) {
    let error_title = &format!("{}", msg);
    common_error(msg.to_string(), position.line, path);
}
pub fn undefined_var(msg: String, source: &str, path: &str, position: TextPosition) {
    let error_title = &format!("{}", msg);
    common_error(msg.to_string(), position.line, path);
}
pub fn undefined_func(msg: String, source: &str, path: &str, position: TextPosition) {
    let error_title = &format!("{}", msg);
    common_error(msg.to_string(), position.line, path);
}
pub fn array_element_wrong_type(
    array_name: String,
    element_index: usize,
    expected_type: BUILTIN_TYPES,
    found_type: BUILTIN_TYPES,
    source: &str,
    path: &str,
    line: usize,
    col: usize
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
        format!(
            "Definējot sarakstu `{}`, tā sākotnējā vērtība pozīcijā {} ir ar nepareizu datu tipu. Nepieciešams {}, bet atrasts {}.",
            array_name,
            element_index,
            expected,
            found
        ),
        line,
        path
    );
}
pub fn array_element_wrong_type_index(
    array_name: String,
    expected_type: BUILTIN_TYPES,
    found_type: BUILTIN_TYPES,
    source: &str,
    path: &str,
    line: usize,
    col: usize
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
        format!(
            "Nepareizs indeksa datu tips, indeksējot sarakstu `{}`. Nepieciešams {}, bet atrasts {}.",
            array_name,
            expected,
            found
        ),
        line,
        path
    );
}
pub fn array_element_index_too_high(
    array_name: String,
    expected_index: usize,
    found_index: usize,
    source: &str,
    path: &str,
    line: usize
) {
    common_error(
        format!(
            "Sarakstu `{}` garums ir {}, bet ir mēģinājums to indeksēt ar indeksu {}.\nSarakstu indeksi tiek skaitīti no nulles.",
            array_name,
            expected_index,
            found_index
        ),
        line,
        path
    );
}

fn common_error(msg: String, line: usize, path: &str) {
    println!("{}\n{}\nFaila \"{}\"\n{}. rindiņā", "Kļūda: ".red(), msg.red(), path, line);
    exit(0);
}
