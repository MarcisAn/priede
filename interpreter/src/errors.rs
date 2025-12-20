use celsium::{ compiletime_helper::{ CompileTimeHelper }, BuiltinTypes };
use colored::Colorize;
use hime_redist::text::TextPosition;

use crate::{ compiler::CompileTimeError, util::{ self, str_from_data_type } };

#[derive(Clone, Debug)]
pub enum CompileTimeErrorType {
    VariableAlreadyDefined {
        varname: String,
    },
    VariableAlreadyIncluded {
        varname: String,
    },
    VariableNotDefined {
        varname: String,
    },
    FunctionNotDefined {
        funcname: String,
    },
    WrongFunctionArgumentCount {
        function_name: String,
        expected_count: usize,
        found_count: usize,
    },
    WrongFunctionArgumentType {
        function_name: String,
        arg_index: usize,
        expected_type: BuiltinTypes,
        found_type: BuiltinTypes,
    },
    NonExistantDataType {
        type_name: String,
    },
    WrongTypeOnArrayDefinition {
        array_name: String,
        element_index: usize,
        expected_type: BuiltinTypes,
        found_type: BuiltinTypes,
    },
    ArrayIndexedWithWrongType {
        found_type: BuiltinTypes,
    },
    ValueNotIndexable {
        found_type: BuiltinTypes,
    },
    WrongVariableInitValue {
        varname: String,
        expected_type: BuiltinTypes,
        found_type: BuiltinTypes,
    },
    BinopNotPossible {
        left: BuiltinTypes,
        right: BuiltinTypes,
    },
    ParserError {
        unexpected: String,
    }

}

pub fn get_message(error: &CompileTimeErrorType) -> String {
    match error {
        CompileTimeErrorType::ParserError { unexpected } =>
            format!("Neparedzēts simbols `{}`", unexpected),
        CompileTimeErrorType::VariableAlreadyDefined { varname } =>
            format!("Mainīgais `{}` jau ir definēts.", varname),
        CompileTimeErrorType::VariableAlreadyIncluded { varname } =>
            format!("Mainīgais `{}` jau ir iekļauts.", varname),
        CompileTimeErrorType::VariableNotDefined { varname } =>
            format!("Mainīgais ar nosaukumu `{}` nav definēts šajā blokā.", varname),
        CompileTimeErrorType::FunctionNotDefined { funcname } =>
            format!("Funkcija ar nosaukumu `{}` nav definēts šajā blokā.", funcname),
        CompileTimeErrorType::WrongFunctionArgumentCount {
            function_name,
            expected_count,
            found_count,
        } =>
            format!(
                "Funkcija {} sagaida {} argumentu{}, bet šeit tiek padot{} {} argument{}",
                function_name,
                expected_count,
                if *expected_count == 1 {
                    ""
                } else {
                    "s"
                },
                if *found_count == 1 {
                    "s"
                } else {
                    "i"
                },
                found_count,
                if *found_count == 1 {
                    "s"
                } else {
                    "i"
                }
            ),
        CompileTimeErrorType::WrongFunctionArgumentType {
            function_name,
            arg_index,
            expected_type,
            found_type,
        } =>
            format!(
                "Funkcija `{}` kā {}. argumentu sagaida datu tipu `{}`, bet atrasts arguments ar tipu `{}`.",
                function_name,
                arg_index,
                str_from_data_type(&expected_type),
                str_from_data_type(&found_type)
            ),
        CompileTimeErrorType::NonExistantDataType { type_name } =>
            format!("Datu tips `{}` neeksistē. Ne kā vienkāršais tips, ne kā objekts.", type_name),
        CompileTimeErrorType::WrongTypeOnArrayDefinition {
            array_name,
            element_index,
            expected_type,
            found_type,
        } =>
            format!(
                "Definējot sarakstu `{}`, tā sākotnējā vērtība pozīcijā `{}` ir ar nepareizu datu tipu. Nepieciešams `{}`, bet atrasts `{}`.",
                array_name,
                element_index,
                str_from_data_type(&expected_type),
                str_from_data_type(&found_type)
            ),
        CompileTimeErrorType::ArrayIndexedWithWrongType { found_type } =>
            format!(
                "Nav iespējams indeksēt sarakstu ar `{}` datu tipu.",
                str_from_data_type(&found_type)
            ),
        CompileTimeErrorType::WrongVariableInitValue { varname, expected_type, found_type } =>
            format!(
                "Nepareizs datu tips mainīgā `{}` sākotnējai vērtībai. Sagaidītais tips ir: `{}`, bet tika mēģināts piešķirt vērtību ar tipu: `{}`",
                varname,
                util::str_from_data_type(&expected_type),
                util::str_from_data_type(&found_type)
            ),
        CompileTimeErrorType::BinopNotPossible { left, right } =>
            format!(
                "Nav iespējams veikt šo darbību starp datu tipiem `{}` un `{}`",
                util::str_from_data_type(&left),
                util::str_from_data_type(&right)
            ),
        CompileTimeErrorType::ValueNotIndexable { found_type } =>
            format!("Datu tips `{}` Nav indeksējams", util::str_from_data_type(&found_type)),

    }
}

pub fn print_error(error: &CompileTimeError, compilehelper: &mut CompileTimeHelper) {
    common_error(&get_message(&error.error_type), error.position, compilehelper);
}

pub fn common_error(
    msg: &str,
    position: Option<TextPosition>,
    compilehelper: &mut CompileTimeHelper
) {
    let path = &compilehelper.source_file_paths[compilehelper.current_file];
    println!(
        "{}\n     {}\n     Faila \"{}\"\n     {}. rindiņā",
        "-----Kļūda: ".red(),
        msg.red(),
        path,
        if position.is_some() {
            position.unwrap().line.to_string()
        } else {
            "".to_string()
        }
    );
    // exit(0);
}
