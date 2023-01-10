use crate::truth_table::TruthTable;
use crate::implicant::Implicant;
use crate::table_parser::convert_boolean_row_to_number;
use crate::groups_structure::GroupStructure;
use prettytable::{Table, Row, Cell, Attr};

const TABULATION_SIZE: u8 = 4;

type CoverageMap = Vec<Vec<bool>>;

pub fn algorithm(table: TruthTable) {
    let mut first_implicants: Vec<Implicant> = Vec::new();
    for (index, row) in table.input_rows().iter().enumerate() {
        if table.row_value(index) {
            first_implicants.push(Implicant::from_input(row, table.variables_names()));
        }
    }

    println!("Function is defined by the unoptimized expression:");
    println!("{}", assemble_expression(&first_implicants));

    println!("\nBeggining iterative optimization by Quine-McCluskey algorithm.");
    println!("Primes found will be marked with an *.");
    let amount_of_variables = table.amount_of_variables();
    let mut groups = agroup(first_implicants.clone(), amount_of_variables);
    groups.print_group();

    let mut iteration: usize = 1;
    loop {
        println!("\nIteration {iteration}");
        let should_continue = groups.combination_step();
        groups.print_group();
        iteration += 1;
        if ! should_continue { break }
    }

    println!("\nAll prime implicants were found. We will now search for the essential ones.");
    println!("This is the coverage map for all primes:");
    let primes = groups.extract_primes();
    let coverage_map = produce_coverage_map(&primes, &first_implicants);
    print_coverage_map(&coverage_map, &primes, &first_implicants);
}

fn agroup(implicants: Vec<Implicant>, amount_of_variables: usize) -> GroupStructure {
    let mut groups = GroupStructure::new(amount_of_variables);
    for implicant in implicants {
        groups.add_implicant(implicant);
    }

    groups
}

fn produce_coverage_map(
    prime_implicants: &Vec<&Implicant>, basic_implicants: &Vec<Implicant>
) -> CoverageMap {
    let mut map: CoverageMap = Vec::new();

    for prime in prime_implicants {
        let mut row = Vec::new();
        for minterm in basic_implicants {
            row.push(prime.covers(minterm));
        }
        map.push(row);
    }

    map
}

fn print_coverage_map(map: &CoverageMap, primes: &Vec<&Implicant>, minterms: &Vec<Implicant>) {
    let mut table = Table::new();

    let mut header_row = Row::empty();
    header_row.add_cell(Cell::new(""));
    for minterm in minterms {
        header_row.add_cell(Cell::new(format!("m{}", minterm.minterm_number()).as_str()));
    }
    table.add_row(header_row);

    for (row_index, prime) in primes.iter().enumerate() {
        let mut row = Vec::new();
        row.push(Cell::new(prime.get_string_representation().as_str()));

        for cover in &map[row_index] {
            row.push(
                // style_spec("c") sets alignment to center
                Cell::new(if *cover { "X" } else { " " }).style_spec("c")
            );
        }

        table.add_row(Row::new(row));
    }

    table.printstd();
}

fn assemble_expression(implicants: &Vec<Implicant>) -> String {
    implicants.iter()
        .map(|implicant| implicant.get_string_representation())
        .collect::<Vec<String>>()
        .join(" + ")
}
