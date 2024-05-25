use annotate_snippets::{Level, Renderer, Snippet};
use anstream;
use celsium::BUILTIN_TYPES;
use colored::Colorize;

pub fn parser_error(unexpected: String, source: &str, path: &str, line: usize, col: usize) {
    let error_title = if unexpected == "saraksts" {
        format!(
            "NEATPAZĪTS SIMBOLS `{}`\nIespējams aizmirsi norādīt saraksta elementu datu tipu",
            unexpected
        )
    } else {
        format!("NEATPAZĪTS SIMBOLS `{}`", unexpected)
    };
    common_error(error_title.to_string(), line, path);
}
pub fn math_error(msg: &str, source: &str, path: &str, line: usize, col: usize) {
    let error_title = &format!("{}", msg);
    common_error(msg.to_string(), line, path);
}
pub fn incorect_init_value(msg: String, source: &str, path: &str, line: usize, col: usize) {
    let error_title = &format!("{}", msg);
    common_error(msg.to_string(), line, path);
}
pub fn undefined_var(msg: String, source: &str, path: &str, line: usize, col: usize) {
    let error_title = &format!("{}", msg);
    common_error(msg.to_string(), line, path);
}
pub fn array_element_wrong_type(
    array_name: String,
    element_index: usize,
    expected_type: BUILTIN_TYPES,
    found_type: BUILTIN_TYPES,
    source: &str,
    path: &str,
    line: usize,
    col: usize,
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
    common_error(format!("Definējot sarakstu `{}`, tā sākotnējā vērtība pozīcijā {} ir ar nepareizu datu tipu. Nepieciešams {}, bet atrasts {}.", array_name, element_index, expected, found), line, path);

}
fn common_error(msg: String, line: usize, path: &str) {
    println!(
        "{}\n{}\nFaila \"{}\"\n{}. rindiņā",
        "Kļūda: ".red(),
        msg.red(),
        path,
        line
    );
}
