#[cfg(target_family = "wasm")]
use crate::console_log;
use crate::IS_WASM;

pub fn print(i: String) {
    unsafe {
        if IS_WASM {
            #[cfg(target_family = "wasm")]
            console_log(&i);
        } else {
            print!("{}", i);
        }
    }
}
pub fn printnl(i: String) {
    unsafe {
        if IS_WASM {
            #[cfg(target_family = "wasm")]
            console_log(&i);
        } else {
            print!("\n{}", i);
        }
    }
}
