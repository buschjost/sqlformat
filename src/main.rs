use atty::Stream;
use sqlformat::*;
use std::env;
use std::io::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const REPOSITORY: &'static str = env!("CARGO_PKG_REPOSITORY");

enum Command {
    Help,
    Version,
    Format,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut command = Command::Format;

    let mut i = 1;
    while i < args.len() {
        if args[i] == "-h" || args[i] == "--help" {
            command = Command::Help;
        } else if args[i] == "-v" || args[i] == "--version" {
            command = Command::Version;
        }
        i += 1;
    }

    match command {
        Command::Help => help(),
        Command::Version => version(),
        Command::Format => format_sql(),
    }
}

fn help() {
    version();
    println!(
        "    
Simple SQL formatter. Reads from stdin and writes to stdout.

{:>1}

USAGE:
    sqlformat

EXAMPLES:
    echo 'SELECT 1 FROM foo;' | sqlformat
    sqlformat < foo.sql

FLAGS:
    -h, --help       Prints this help information
    -v, --version    Prints version information",
        REPOSITORY
    );
}

fn version() {
    println!("sqlformat {:>1}", VERSION);
}

fn format_sql() {
    if atty::is(Stream::Stdin) {
        help();
        return;
    }

    let mut buffer = String::new();
    std::io::stdin()
        .read_to_string(&mut buffer)
        .unwrap_or_else(|error| {
            panic!("Problem reading stdin: {:?}", error);
        });

    let formatted = format(&buffer, &QueryParams::None, &FormatOptions::default());

    println!("{}", formatted);
}
