use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn priede_print(s: &str);
}

#[wasm_bindgen]
pub fn run(code: &str) {
    #[cfg(target_family = "wasm")]
    priede_print(code);
    interpreter::run_wasm(code.to_string());
}
