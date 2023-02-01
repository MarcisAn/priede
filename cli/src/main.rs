pub fn main() {
    let arguments = std::env::args();
    let arguments = arguments::parse(arguments).unwrap();
    //print!("{}", arguments.get::<bool>("bar").unwrap());
    if arguments.orphans.len() == 0 {
        interpreter::interpret(false, String::from("../examples/sveika_pasaule.pr"), false);
    } else {
        let mut print_ast: bool = false;
        if arguments.orphans.len() == 2 {
            if arguments.orphans[1] == "ast" {
                print_ast = true;
            }
        }
        interpreter::interpret(print_ast, String::from(arguments.orphans[0].clone()), false);
    }
}
