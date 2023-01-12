extern crate core;

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

fn main() {
    let mut args = env::args();
    args.next(); // discard program name arg
    let provided_filepath = args.next();

    let filepath = match provided_filepath {
        Some(path) => path,
        None => panic!("Please, provide a filepath to a truth table (has to be .csv).")
    };

    let mut step_by_step = false;

    for arg in args {
        if arg.as_str() == "--step-by-step" {
            step_by_step = true;
        }
    }

    let table = TruthTable::from_csv(filepath.as_str());
    algorithm(table, step_by_step);
}
