use celsium::compiletime_helper::{ CompileTimeHelper, CompilerError };
use hime_redist::errors::{
    ParseError,
    ParseErrorDataTrait,
    ParseErrorUnexpectedToken,
};

fn unexpected_token_error(
    err: ParseErrorUnexpectedToken,
    compilehelper: &mut CompileTimeHelper
) -> CompilerError {
    let unexpected_token = err.clone().value;
    let position = err.get_position();

    let error = CompilerError {
        message: format!("Neparedzēts simbols {}", unexpected_token),
        line: Some(position.line),
        file: compilehelper.source_file_paths[compilehelper.current_file].clone(),
    };
    crate::errors::parser_error(unexpected_token, err.get_position(), compilehelper);
    return error;
}

pub fn parser_errors(
    errors: Vec<ParseError>,
    compilehelper: &mut CompileTimeHelper
) -> Vec<CompilerError> {
    let mut result_errors = vec![];
    for parse_err in errors.clone() {
        match parse_err {
            hime_redist::errors::ParseError::UnexpectedToken(err) =>
                result_errors.push(unexpected_token_error(err, compilehelper)),
            _ => (),
        }
    }
    result_errors
}
