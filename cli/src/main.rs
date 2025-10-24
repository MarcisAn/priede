use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    program_file: Vec<String>,

    /// Number of times to greet
    #[arg(short, long, default_value_t = false)]
    ast: bool,

    #[arg(short, long, default_value_t = false)]
    bytecode: bool,

    #[arg(short, long, default_value_t = false)]
    format: bool,

    #[arg(short, long, default_value_t = false)]
    static_only: bool,

    #[arg(short, long, default_value_t = false)]
    testing_stack: bool,
}

pub fn main() {
    // let arguments = std::env::args();
    // let arguments = arguments::parse(arguments).unwrap();
    // println!("{}", arguments.orphans[0]);
    let args = Args::parse();
    if args.format {
        let formated= interpreter::formater::format(interpreter::read_file(args.program_file[0].to_string()), args.ast);
        println!("{}", formated);
    } else {
        for file in 0..args.program_file.len() {
            interpreter::interpret(
                args.program_file[file].to_string(),
                args.ast,
                args.bytecode,
                args.static_only,
                args.testing_stack
            );
        }
    }
    // let mut halt = false;
    // let mut verbose: u8 = 0;
    //     let mut static_analysis = false;
    //     if arguments.get::<u8>("verbose").is_some() {
    //         verbose = arguments.get::<u8>("verbose").unwrap();
    //     }
    //     if arguments.get::<bool>("static").is_some() {
    //         static_analysis = arguments.get::<bool>("static").unwrap();
    //     }
    //     if arguments.get::<bool>("help").is_some() {
    //         println!("Priedes dokumentācija: https://priede.andersons-m.lv");
    //         return;
    //     }
    // if arguments.orphans.len() == 0 {
    //     //quick debuging way
    //     let res = interpreter::interpret(String::from("../examples/sveika_pasaule.pr"), 3, false);
    //     // println!("{}", interpreter::format(fs::read_to_string("../examples/sveika_pasaule.pr").unwrap()));
    // } else {

    //     interpreter::interpret(String::from(arguments.orphans[0].clone()), verbose, static_analysis);
    //     if halt {
    //         let mut stdin = io::stdin();
    //         let mut stdout = io::stdout();
    //         write!(stdout, "Programmas izpilde veiksmīga. Šo logu var aizvērt.").unwrap();
    //         stdout.flush().unwrap();
    //         let _ = stdin.read(&mut [0u8]).unwrap();
    //     }
    // }
}
