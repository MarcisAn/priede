use hime_redist::errors::{ ParseError, ParseErrorDataTrait, ParseErrorUnexpectedToken };

use crate::compiler::Compiler;

fn unexpected_token_error(err: ParseErrorUnexpectedToken, compiler: &mut Compiler) {
    let unexpected_token = err.clone().value;
    let path = &compiler.helper.source_file_paths[compiler.helper.current_file];
    let position = err.get_position();
    
    compiler.add_parser_error(
        unexpected_token,
        position,
        path.to_string()
    );
}

pub fn parser_errors(errors: Vec<ParseError>, compiler: &mut Compiler) {
    for parse_err in errors.clone() {
        match parse_err {
            hime_redist::errors::ParseError::UnexpectedToken(err) =>
                unexpected_token_error(err, compiler),
            _ => (),
        }
    }
}
