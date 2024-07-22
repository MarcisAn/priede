use std::io;
use std::io::prelude::*;


pub fn main() {
    let arguments = std::env::args();
    let arguments = arguments::parse(arguments).unwrap();
    let mut halt = false;
    let mut verbose: u8 = 0;
        let mut static_analysis = false;
        if arguments.get::<u8>("verbose").is_some() {
            verbose = arguments.get::<u8>("verbose").unwrap();
        }
        if arguments.get::<bool>("static").is_some() {
            static_analysis = arguments.get::<bool>("static").unwrap();
        }
        if arguments.get::<bool>("help").is_some() {
            println!("Priedes dokumentācija: https://priede.andersons-m.lv");
            return;
        }
    if arguments.orphans.len() == 0 {
        //quick debuging way
        let res = interpreter::interpret(String::from("../examples/sveika_pasaule.pr"), 3, false);
        //println!("res: {:?}", res);
    } else {
        
        interpreter::interpret(String::from(arguments.orphans[0].clone()), verbose, static_analysis);
        if halt {
            let mut stdin = io::stdin();
            let mut stdout = io::stdout();
            write!(stdout, "Programmas izpilde veiksmīga. Šo logu var aizvērt.").unwrap();
            stdout.flush().unwrap();
            let _ = stdin.read(&mut [0u8]).unwrap();
        }
    }
}