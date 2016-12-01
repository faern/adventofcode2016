#[macro_use]
extern crate clap;

extern crate day1;

use clap::{Arg, App};
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use std::process;


static APP_NAME: &'static str = "Advent of Code 2016 CLI";
static APP_VERSION: &'static str = "0.0.0";
static APP_AUTHOR: &'static str = "Linus Färnstrand <faern@faern.net>";
static APP_ABOUT: &'static str = "Run Advent of Code solutions";

/// Macro for printing to stderr. Will simply do nothing if the printing fails for some reason.
macro_rules! eprintln {
    ($($arg:tt)*) => (
        use std::io::Write;
        let _ = writeln!(&mut ::std::io::stderr(), $($arg)* );
    )
}

fn main() {
    let app = create_app();
    let matches = app.clone().get_matches();

    let problem = matches.value_of("problem").unwrap();
    let input_path = matches.value_of("input").unwrap();

    let mut input_data = String::new();
    {
        let mut f = File::open(input_path).unwrap();
        f.read_to_string(&mut input_data).unwrap();
    }

    let timer = Instant::now();
    match day1::solve(input_data) {
        Ok(solution) => {
            let time = timer.elapsed();
            println!("Solution: {}\nTime to solve: {:?}", solution, time);
        },
        Err(e) => {
            eprintln!("Unable to solve problem {}: {}", problem, e);
            process::exit(1);
        }
    }
}

fn create_app() -> App<'static, 'static> {
    App::new(APP_NAME)
        .version(APP_VERSION)
        .author(APP_AUTHOR)
        .about(APP_ABOUT)
        .arg(Arg::with_name("problem")
            .short("-p")
            .long("problem")
            .help("Select which problem to solve.")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .help("Specify what problem input file to use.")
            .takes_value(true)
            .required(true))
}
