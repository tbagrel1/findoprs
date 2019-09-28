use clap::{App, Arg};
use std::path::Path;

const DEFAULT_NAME_RADICAL: &'static str = "operations";
const DEFAULT_GENERATE_ANSWERS: bool = true;

fn main() {
    let args = App::new("FindOp Rust")
        .author("Thomas Bagrel <tomsb07@gmail.com>")
        .about("CLI to generate maths exercices")
        .arg(Arg::with_name("OUTPUT_DIR")
            .required(true)
            .takes_value(true)
            .help("Where files will be generated")
        )
        .arg(Arg::with_name("name_radical")
            .short("n")
            .long("name")
            .required(false)
            .takes_value(true)
            .default_value(DEFAULT_NAME_RADICAL)
            .help("Name radical for generated files"))
        .arg(Arg::with_name("generate_answers")
            .short("g")
            .long("generate-answers")
            .required(false)
            .takes_value(false)
            .help("Should answers be generated too?")
        )
        .arg(Arg::with_name("add")
            .short("a")
            .long("add")
            .required(false)
            .takes_value(false)
            .help("Generate additions"))
        .arg(Arg::with_name("subtract")
            .short("s")
            .long("subtract")
            .required(false)
            .takes_value(false)
            .help("Generate subtractions"))
        .arg(Arg::with_name("multiply")
            .short("m")
            .long("multiply")
            .required(false)
            .takes_value(false)
            .help("Generate multiplications"))
        .arg(Arg::with_name("divide")
            .short("d")
            .long("divide")
            .required(false)
            .takes_value(false)
            .help("Generate divisions"))
        .get_matches();

    let output_dir = Path::new(args.value_of("OUTPUT_DIR").unwrap());
    let name_radical = args.value_of("name_radical").unwrap();
    let generate_answers = args.is_present("generate_answers");
    let generate_add = args.is_present("add");
    let generate_sub = args.is_present("subtract");
    let generate_mul = args.is_present("multiply");
    let generate_div = args.is_present("divide");

    println!("Hello, world!");
}
