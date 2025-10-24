use celsium::compiletime_helper::{CompileTimeHelper, CompilerError};
use hime_redist::errors::{
    ParseError,
    ParseErrorDataTrait,
    ParseErrorUnexpectedChar,
    ParseErrorUnexpectedToken,
};


fn unexpected_token_error(
    err: ParseErrorUnexpectedToken,
    compilehelper: &mut CompileTimeHelper
) -> CompilerError {
    let mut err_str = err.to_string();
    let expected_start = err.to_string().find("\"").unwrap();
    let mut split = err_str.split_off(expected_start);
    let _ = split.split_off(split.len() - 8);
    let unexpected_token = split.split_off(1);
    let position = err.get_position();

    let error = CompilerError{ message: format!("Neparedzēts simbols {}", unexpected_token), line: Some(position.line), file: compilehelper.source_file_paths[compilehelper.current_file].clone() };
    crate::errors::parser_error(unexpected_token, err.get_position(), compilehelper);
    return error;
}
fn unexpected_char_error(
    err: ParseErrorUnexpectedChar,
    compilehelper: &mut CompileTimeHelper
) -> CompilerError {
    let mut err_str = err.to_string();
    let expected_start = err.to_string().find("'").unwrap();
    let mut split = err_str.split_off(expected_start);
    let _ = split.split_off(split.len() - 8);
    let unexpected_token = split.split_off(1);
    let position = err.get_position();

    let error = CompilerError{ message: format!("Neparedzēts simbols {}", unexpected_token), line: Some(position.line), file: compilehelper.source_file_paths[compilehelper.current_file].clone() };
    crate::errors::parser_error(unexpected_token, err.get_position(), compilehelper);
    return error;
}

pub fn parser_errors(
    errors: Vec<ParseError>,
    compilehelper: &mut CompileTimeHelper
) -> Vec<CompilerError> {
    let mut result_errors = vec![];
    for parse_err in errors.clone() {
        let error = match parse_err {
            hime_redist::errors::ParseError::UnexpectedEndOfInput(_) => todo!(),
            hime_redist::errors::ParseError::UnexpectedChar(err) =>
                unexpected_char_error(err, compilehelper),
            hime_redist::errors::ParseError::UnexpectedToken(err) =>
                unexpected_token_error(err, compilehelper),
            hime_redist::errors::ParseError::IncorrectUTF16NoLowSurrogate(_) => todo!(),
            hime_redist::errors::ParseError::IncorrectUTF16NoHighSurrogate(_) => todo!(),
        };
        result_errors.push(error);
    }
    result_errors
}
