extern crate core;

mod table_parser;
mod implicant;
mod truth_table;
mod algorithm;
mod groups_structure;
mod coverage_map;

use truth_table::TruthTable;
use algorithm::algorithm;

fn main() {
    let table = TruthTable::from_csv("example_tables/func-a.csv");

    algorithm(table);
}


