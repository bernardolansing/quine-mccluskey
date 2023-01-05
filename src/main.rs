mod table_parser;

use table_parser::read_csv;

fn main() {
    let table = read_csv("example_tables/func-a.csv");
    table.print_table();
}
