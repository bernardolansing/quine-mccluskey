use crate::truth_table::TruthTable;
use crate::implicant::Implicant;
use crate::table_parser::convert_boolean_row_to_number;
use crate::groups_structure::GroupStructure;
use crate::coverage_map::CoverageMap;
use prettytable::{Table, Row, Cell, Attr, AsTableSlice};

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
    let mut coverage_map = CoverageMap::new(&primes, &first_implicants);
    coverage_map.print();

    println!("\nEssential primes were marked in green:");
    coverage_map.find_essentials();
    coverage_map.print();
}

fn agroup(implicants: Vec<Implicant>, amount_of_variables: usize) -> GroupStructure {
    let mut groups = GroupStructure::new(amount_of_variables);
    for implicant in implicants {
        groups.add_implicant(implicant);
    }

    groups
}

fn assemble_expression(implicants: &Vec<Implicant>) -> String {
    implicants.iter()
        .map(|implicant| implicant.get_string_representation())
        .collect::<Vec<String>>()
        .join(" + ")
}
