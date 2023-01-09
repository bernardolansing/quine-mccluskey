mod table_parser;
mod implicant;
mod truth_table;
mod algorithm;

use table_parser::read_csv;
use truth_table::TruthTable;
use crate::implicant::Implicant;
use algorithm::algorithm;



fn main() {
    let table = TruthTable::from_csv("example_tables/func-a.csv");

    algorithm(table);
}


