use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn wasm_print(s: &str);
    fn wasm_input(s: &str);

}

#[wasm_bindgen]
pub fn run(code: &str) {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    interpreter::run_wasm(code.to_string());
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn it_works() {
        let input = "
        teksts a : \"Sveika, pasaule!\" \
        \
        drukāt(a)";
        run(input);
    }
}