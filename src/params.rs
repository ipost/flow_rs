use clap::{App, Arg};
use std::path::PathBuf;

pub struct Parameters {
    pub print_ast: bool,
    pub in_file: Option<PathBuf>,
    pub out_file: Option<PathBuf>,
}

fn get_parameters() -> clap::ArgMatches<'static> {
    App::new("flow_rs")
        .version("0.1.0")
        .author("Isaac Post <post.isaac@gmail.com>")
        .about("about")
        .arg(
            Arg::with_name("PRINT_AST")
                .short("a")
                .long("print-ast")
                .help("Print abstract syntax tree"),
        )
        .arg(
            Arg::with_name("IN_FILE")
                .short("f")
                .long("input-file")
                .takes_value(true)
                .help("The input .flowrs file, reads from STDIN by default"),
        )
        .arg(
            Arg::with_name("OUT_FILE")
                .short("o")
                .long("output-file")
                .takes_value(true)
                .help("The output .dot file, writes to STDOUT by default"),
        )
        .get_matches()
}

pub fn parameters() -> Parameters {
    let params = get_parameters();
    let print_ast = params
        .is_present("PRINT_AST");
    let in_file = match params.value_of("IN_FILE") {
        Some(f) => Some(PathBuf::from(f)),
        None => None,
    };
    let out_file = match params.value_of("OUT_FILE") {
        Some(f) => Some(PathBuf::from(f)),
        None => None,
    };
    Parameters {
        print_ast,
        in_file,
        out_file,
    }
}
