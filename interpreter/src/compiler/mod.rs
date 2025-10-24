use celsium::{
    compiletime_helper::CompileTimeHelper,
    module::Function,
    typestack::TypeStack,
};




pub struct Compiler {
    pub helper: CompileTimeHelper,
    pub typestack: TypeStack,
    pub is_wasm: bool,
    pub functions: Vec<Function>,
}
