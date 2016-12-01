#![cfg_attr(feature = "bench", feature(test))]

#[macro_use]
extern crate clap;

#[cfg(feature = "bench")]
extern crate test;

extern crate base;
extern crate day1;

use base::{Part, ProblemSolver};

use clap::{Arg, App};
use std::fs::File;
use std::io::{self, Read};
use std::process;
use std::str::FromStr;
use std::time::{Instant, Duration};

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
    let (day, part, input_path, bench) = parse_arguments().unwrap_or_else(|e| {
        eprintln!("Unable to parse arguments: {}", e);
        process::exit(1);
    });
    let input = read_input(&input_path).unwrap_or_else(|e| {
        eprintln!("Unable to read input from {}: {}", input_path, e);
        process::exit(1);
    });
    let solver = get_problem_solver(day).unwrap_or_else(|e| {
        eprintln!("Error with problem solver: {}", e);
        process::exit(1);
    });

    if bench {
        benchmark(solver, part, input);
    } else {
        solve(solver, day, part, input);
     }
}

fn solve(solver: Box<ProblemSolver>, day: u8, part: Part, input: String) {
    let solution_timer = Instant::now();
    let solution = solver.solve(part, input).unwrap_or_else(|e| {
        eprintln!("Unable to solve problem {}.{}: {}", day, part, e);
        process::exit(1);
    });

    let time = solution_timer.elapsed();
    println!("Solution: {}\nTime to solve: {}",
             solution,
             format_duration(&time));
}

#[cfg(feature = "bench")]
fn benchmark(solver: Box<ProblemSolver>, part: Part, input: String) {
    use test::{fmt_bench_samples, bench, black_box, Bencher};
    let samples = bench::benchmark(|b: &mut Bencher| {
        b.iter(|| {
            solver.solve(part, black_box(input.clone())).unwrap()
        })
    });
    println!("{}", fmt_bench_samples(&samples));
}

#[cfg(not(feature = "bench"))]
fn benchmark(_solver: Box<ProblemSolver>, _part: Part, _input: String) {}

fn parse_arguments() -> Result<(u8, Part, String, bool), String> {
    let app = create_app();
    let matches = app.clone().get_matches();

    let day = value_t!(matches.value_of("day"), u8).unwrap();
    if day < 1 || day > 25 {
        return Err("Day must be 1-25".to_owned());
    }
    let part = Part::from_str(matches.value_of("part").unwrap())?;
    let input_path = matches.value_of("input").unwrap().to_owned();

    let bench = matches.is_present("bench");

    Ok((day, part, input_path, bench))
}

fn read_input(input_path: &str) -> io::Result<String> {
    let mut input_data = String::new();
    let mut f = File::open(input_path)?;
    f.read_to_string(&mut input_data)?;
    Ok(input_data)
}

fn get_problem_solver(day: u8) -> Result<Box<ProblemSolver>, String> {
    match day {
        1 => Ok(day1::get_solver()),
        _ => Err(format!("No solver for day {}", day)),
    }
}

fn format_duration(duration: &Duration) -> String {
    let us_small = duration.subsec_nanos() as u64 / 1_000;
    let us_large = duration.as_secs() * 1_000_000;
    let combined = us_small + us_large;
    format!("{} us", combined)
}

fn create_app() -> App<'static, 'static> {
    let app = App::new(APP_NAME)
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
            .required(true));
    add_nightly_args(app)
}

#[cfg(feature = "bench")]
fn add_nightly_args(app: App<'static, 'static>) -> App<'static, 'static> {
    app.arg(Arg::with_name("bench")
        .long("bench")
        .help("Activate benchmarking mode instead of just solving."))
}

#[cfg(not(feature = "bench"))]
fn add_nightly_args(app: App<'static, 'static>) -> App<'static, 'static> {
    app
}
