extern crate core;

mod table_parser;
mod implicant;
mod truth_table;
mod algorithm;
mod groups_structure;
mod coverage_map;

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
    let table = TruthTable::from_csv(filepath.as_str());

    algorithm(table);
}
