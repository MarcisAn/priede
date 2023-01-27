use crate::{console_log, IS_WASM};

pub fn print(i: String) {
    unsafe {
        if IS_WASM {
            console_log(&i);
        } else {
            print!("{}", i);
        }
    }
}
pub fn printnl(i: String) {
    unsafe {
        if IS_WASM {
            console_log(&i);
        } else {
            print!("\n{}", i);
        }
    }
}
