use std::io;
use std::io::prelude::*;

pub fn main() {
    let arguments = std::env::args();
    let arguments = arguments::parse(arguments).unwrap();
    let mut halt = false;
    if arguments.orphans.len() == 0 {
        let res = interpreter::interpret(String::from("../examples/sveika_pasaule.pr"), 3, false);
        //println!("res: {:?}", res);
    } else {
        let mut verbose: u8 = 0;
        if arguments.orphans.len() >= 2 {
            if arguments.orphans[1] == "ast" {
                verbose = 2;
            } else if arguments.orphans[1] == "v3" {
                verbose = 3;
            }
            if arguments.orphans.len() >= 3 {
                if arguments.orphans[2] == "halt" {
                    halt = true;
                }
            }
        }
        interpreter::interpret(String::from(arguments.orphans[0].clone()), verbose, false);
        if halt {
            let mut stdin = io::stdin();
            let mut stdout = io::stdout();
            write!(stdout, "Programmas izpilde veiksmīga. Šo logu var aizvērt.").unwrap();
            stdout.flush().unwrap();
            let _ = stdin.read(&mut [0u8]).unwrap();
        }
    }
}
