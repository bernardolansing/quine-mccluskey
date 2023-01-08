mod table_parser;
mod minterm;
mod truth_table;

use table_parser::read_csv;
use truth_table::TruthTable;
use crate::minterm::Implicant;

fn main() {
    let table = TruthTable::from_csv("example_tables/func-a.csv");

    let mut first_implicants: Vec<Implicant> = Vec::new();

    for (index, row) in table.input_rows().iter().enumerate() {
        if table.row_value(index) {
            first_implicants.push(Implicant::new(row, table.variables_names()));
        }
    }

    for implicant in first_implicants {
        println!("{}", implicant.get_string_representation())
    }
}
