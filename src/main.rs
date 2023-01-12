// extern crate core;

mod table_parser;
mod implicant;
mod truth_table;
mod algorithm;
mod groups_structure;
mod coverage_map;

use std::collections::{HashMap, HashSet};
use truth_table::TruthTable;
use algorithm::algorithm;
use std::env;
use std::fs;

fn main() {
    let mut args = env::args();
    args.next(); // discard program name arg
    let provided_filepath = args.next();

    let filepath = match provided_filepath {
        Some(path) => path,
        None => panic!("Please, provide a filepath to a truth table (has to be .csv).")
    };

    let mut step_by_step = false;
    let mut dump_directory: Option<String> = None;

    // iterate over optional args provided
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--step-by-step" => { step_by_step = true }
            "--dump" => {
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

    let table = TruthTable::from_csv(filepath.as_str());
    let result = algorithm(table, step_by_step);

    match dump_directory {
        Some(dir) => {
            fs::write(&dir, result).expect("Error trying to write dump file.");
            println!("\nThis result was dumped into file '{}'.", dir);
        },
        None => {}
    }
}
