use std::env;
use std::path::Path;

fn main() {
    if env::consts::OS == "windows" {
        if Path::new("C://Program Files/Priede").exists() == true {
            println!("Priedes mape jau eksistē");
        } else {
            if Path::new("C://Program Files").exists() == true {
                println!(
                    "Priedes instalācijas vieta ir {}",
                    "C://Program Files/Priede"
                );
            } else {
                println!("Uz datora neeksistē \"C://Program Files\" mape")
            }
        }
    }
}
