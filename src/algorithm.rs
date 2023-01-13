use crate::truth_table::TruthTable;
use crate::implicant::Implicant;
use crate::groups_structure::GroupStructure;
use crate::coverage_map::CoverageMap;
use std::io;
use std::io::{Write};

pub fn algorithm(table: TruthTable, step_by_step: bool) -> String {
    let mut first_implicants: Vec<Implicant> = Vec::new();
    for (index, row) in table.input_rows().iter().enumerate() {
        if table.row_value(index) {
            first_implicants.push(Implicant::from_input(row, table.variables_names()));
        }
    }

    println!("Read table from provided input:");
    table.print_table();

    if step_by_step { wait_for_user_advance() }

    println!("\nFunction is defined by the unoptimized expression:");
    let function_defining_expression = assemble_expression(
        &first_implicants.iter()
            .map(|ref_to_implicant| ref_to_implicant)
            .collect::<Vec<&Implicant>>()
            .as_slice()
    );
    println!("{}", function_defining_expression);

    if step_by_step { wait_for_user_advance() }

    println!("\nBeggining iterative optimization by Quine-McCluskey algorithm.");
    println!("Primes found will be marked with an *.");
    let amount_of_variables = table.amount_of_variables();
    let mut groups = agroup(first_implicants.clone(), amount_of_variables);
    groups.print_group();

    if step_by_step { wait_for_user_advance() }

    let mut iteration: usize = 1;
    loop {
        println!("\nIteration {iteration}");
        let should_continue = groups.combination_step();
        groups.print_group();
        iteration += 1;
        if step_by_step { wait_for_user_advance() }
        if ! should_continue { break }
    }

    println!("\nAll prime implicants were found. We will now search for the essential ones.");
    println!("This is the coverage map for all primes:");
    let primes = groups.extract_primes();
    let mut coverage_map = CoverageMap::new(&primes, &first_implicants);
    coverage_map.print();

    if step_by_step { wait_for_user_advance() }

    let essentials_found = coverage_map.find_essentials();
    println!("\nEssential primes were marked in green ({essentials_found} found):");
    coverage_map.print();

    if step_by_step { wait_for_user_advance() }

    println!("\nArbitrary selection of implicants to cover the remaining minterms:");
    coverage_map.choose_remaining_primes();
    coverage_map.print();

    if step_by_step { wait_for_user_advance() }

    println!(
        "\nOptimization process is finished. An equivalent formula for the provided function is:"
    );
    let selected_implicants_indexes = coverage_map.get_selected_implicants();
    let selected_implicants: Vec<&Implicant> = primes.iter()
        .enumerate()
        .filter(|(index, _)| selected_implicants_indexes.contains(index))
        .map(|(_, prime)| prime)
        .collect();
    let final_formula = assemble_expression(selected_implicants.as_slice());
    println!("{final_formula}");

    final_formula
}

fn agroup(implicants: Vec<Implicant>, amount_of_variables: usize) -> GroupStructure {
    let mut groups = GroupStructure::new(amount_of_variables);
    for implicant in implicants {
        groups.add_implicant(implicant);
    }

    groups
}

fn assemble_expression(implicants: &[&Implicant]) -> String {
    implicants.iter()
        .map(|implicant| implicant.get_string_representation())
        .collect::<Vec<String>>()
        .join(" + ")
}

fn wait_for_user_advance() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "\nPress any key to continue.").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read_line(&mut String::new()).unwrap();
}
