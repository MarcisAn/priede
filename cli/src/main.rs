pub fn main() {
    let arguments = std::env::args();
    let arguments = arguments::parse(arguments).unwrap();
    //print!("{}", arguments.get::<u8>("bar").unwrap());
    if arguments.orphans.len() == 0 {
        interpreter::interpret(String::from("../examples/sveika_pasaule.pr"), 1);
    } else {
        let mut verbose: u8 = 0;
        if arguments.orphans.len() == 2 {
            if arguments.orphans[1] == "ast" {
                verbose = 2;
            }
            else if arguments.orphans[1] == "v3" {
                verbose = 3;
            }
        }
        interpreter::interpret(String::from(arguments.orphans[0].clone()), verbose);
    }
}
