use celsium::compiletime_helper::CompileTimeHelper;
use hime_redist::errors::{
    ParseError,
    ParseErrorDataTrait,
    ParseErrorUnexpectedChar,
    ParseErrorUnexpectedToken,
};

use crate::{ compiler::CompileError, errors, util::rem_first_and_last };

fn unexpected_token_error(
    err: ParseErrorUnexpectedToken,
    compilehelper: &mut CompileTimeHelper
) -> CompileError {
    let mut err_str = err.to_string();
    let expected_start = err.to_string().find("; expected").unwrap();
    let _ = err_str.split_off(expected_start);

    let unexpected_token = rem_first_and_last(&err_str.split_off(17)).to_string();
    let position = err.get_position();

    let error = CompileError {
        line: position.line,
        char_start: position.column,
        length: err.get_length(),
        error_type: crate::compiler::CompileErrorType::Parser {
            unexpected_string: unexpected_token.clone(),
        },
        path: compilehelper.source_file_paths[compilehelper.current_file].clone(),
    };
    crate::errors::parser_error(unexpected_token, err.get_position(), compilehelper);
    return error;
}
fn unexpected_char_error(
    err: ParseErrorUnexpectedChar,
    compilehelper: &mut CompileTimeHelper
) -> CompileError {
    let mut err_str = err.to_string();
    let expected_start = err.to_string().find("'").unwrap();
    let mut split = err_str.split_off(expected_start);
    let _ = split.split_off(split.len() - 8);
    let unexpected_token = split.split_off(1);
    let position = err.get_position();

    let error = CompileError {
        line: position.line,
        char_start: position.column,
        length: err.get_length(),
        error_type: crate::compiler::CompileErrorType::Parser {
            unexpected_string: unexpected_token.clone(),
        },
        path: compilehelper.source_file_paths[compilehelper.current_file].clone(),

    };
    crate::errors::parser_error(unexpected_token, err.get_position(), compilehelper);
    return error;
}

pub fn parser_errors(
    errors: Vec<ParseError>,
    compilehelper: &mut CompileTimeHelper
) -> Vec<CompileError> {
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
