mod table_parser;
mod implicant;
mod truth_table;
mod algorithm;
mod groups_structure;
mod coverage_map;

use truth_table::TruthTable;
use algorithm::algorithm;
use std::env;
use std::fs;
use std::process;

fn main() {
    let mut args = env::args();
    args.next(); // discard program name arg
    let provided_filepath = args.next();

    let first_argument = match provided_filepath {
        Some(path) => path,
        None => panic!("Please, provide a filepath to a truth table (has to be .csv). Consider \
        consulting the help section with quine-mccluskey --help.")
    };

    // if called, this function terminates the program.
    if ["--help", "-h"].contains(&first_argument.as_str()) { print_help() }

    let filepath = first_argument.as_str();

    let mut step_by_step = false;
    let mut dump_directory: Option<String> = None;

    // iterate over optional args provided
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--step-by-step" => { step_by_step = true },
            "--help" | "-h" => print_help(),
            "--dump" | "-d" => {
                let mut provided_directory = args.next()
                    .expect("expected a filepath where dump result to.");

                if ! provided_directory.ends_with(".txt") {
                    provided_directory.push_str(".txt");
                }

                dump_directory = Some(provided_directory.clone());
            }
            _ => {}
        }
    }

    let table = TruthTable::from_csv(filepath);
    let result = algorithm(table, step_by_step);

    match dump_directory {
        Some(dir) => {
            fs::write(&dir, result).expect("Error trying to write dump file.");
            println!("\nThis result was dumped into file '{}'.", dir);
        },
        None => {}
    }
}

fn print_help() {
    println!("\nThe first argument must be the filepath to the truth table you want to optimize.");
    println!("This table needs to be in .csv format, filled with 0s and 1s or Vs and Fs.");
    println!(
        "Please, make sure it is written from the least significant input to the most significant."
    );

    println!("filepath");
    println!("[ --step-by-step ] will run the program pausing after completing every step. \
    User will be prompted to press any key to continue.");
    println!("[ --dump | -d <path> ] writes the result in the provided file location. .txt \
    only. It is not necessary provide the .txt extension when specifying the path.");
    println!("[ -h | --help ] shows this message.");

    process::exit(0);
}
