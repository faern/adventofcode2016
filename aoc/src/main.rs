#[macro_use]
extern crate clap;

extern crate day1;

use clap::{Arg, App};
use std::fs::File;
use std::io::Read;
use std::time::{Instant, Duration};
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
    let (day, part, input_path) = match parse_arguments() {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Unable to parse arguments: {}", e);
            process::exit(1);
        }
    };
    let input = read_input(&input_path);

    let timer = Instant::now();
    match day1::solve(part, input) {
        Ok(solution) => {
            let time = timer.elapsed();
            println!("Solution: {}\nTime to solve: {}", solution, format_duration(&time));
        },
        Err(e) => {
            eprintln!("Unable to solve problem {}.{}: {}", day, part, e);
            process::exit(1);
        }
    }
}

fn parse_arguments() -> Result<(u8, u8, String), String> {
    let app = create_app();
    let matches = app.clone().get_matches();

    let day = value_t!(matches.value_of("day"), u8).unwrap();
    if day < 1 || day > 25 {
        return Err("Day must be 1-25".to_owned());
    }
    let part = value_t!(matches.value_of("part"), u8).unwrap();
    if part < 1 || part > 2 {
        return Err("Part must be 1 or 2".to_owned());
    }
    let input_path = matches.value_of("input").unwrap().to_owned();

    Ok((day, part, input_path))
}

fn read_input(input_path: &str) -> String {
    let mut input_data = String::new();
    let mut f = File::open(input_path).unwrap();
    f.read_to_string(&mut input_data).unwrap();
    input_data
}

fn format_duration(duration: &Duration) -> String {
    let us_small = duration.subsec_nanos() as u64 / 1_000;
    let us_large = duration.as_secs() * 1_000_000;
    let combined = us_small + us_large;
    format!("{} us", combined)
}

fn create_app() -> App<'static, 'static> {
    App::new(APP_NAME)
        .version(APP_VERSION)
        .author(APP_AUTHOR)
        .about(APP_ABOUT)
        .arg(Arg::with_name("day")
            .long("day")
            .help("Select which day's problem to solve.")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("part")
            .long("part")
            .help("Select which part of the problem to solve, 1 or 2.")
            .default_value("1"))
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .help("Specify what problem input file to use.")
            .takes_value(true)
            .required(true))
}
