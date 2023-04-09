use lazy_static::lazy_static;
use std::path;

static mut WASM: bool = false;
//ir tikai viena root direktorija, bet tā ir vekotrā, jo statiskie mainīgie čakarējās
static mut ROOT_DIR: Vec<String> = Vec::new();

pub fn get_int_state() -> (String, bool) {
    unsafe {
        return (ROOT_DIR[0].to_string(), WASM);
    }
}
pub fn set_int_state(isWasm: bool, mut root: String) {
    unsafe {
        WASM = isWasm;
        ROOT_DIR.append(&mut vec![root]);
    }
}
