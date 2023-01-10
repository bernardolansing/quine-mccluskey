use std::borrow::Borrow;
use crate::truth_table::TruthTable;
use crate::implicant::Implicant;
use crate::table_parser::convert_boolean_row_to_number;
use prettytable::{Table, Row, Cell};

const TABULATION_SIZE: u8 = 4;

type CoverageMap = Vec<Vec<bool>>;

struct GroupStructure {
    amount_of_groups: usize,
    groups: Vec<Vec<Implicant>>
}

impl GroupStructure {
    fn new(amount_of_variables: usize) -> Self {
        let mut groups: Vec<Vec<Implicant>> = Vec::new();
        // +1 because a function with n variables can have n + 1 different amounts of true values
        (0..amount_of_variables + 1).for_each(|_| groups.push(Vec::new()));

        GroupStructure { groups, amount_of_groups: amount_of_variables + 1 }
    }

    fn add_implicant(&mut self, implicant: Implicant) {
        let index = implicant.amount_of_true_variables();
        self.groups[index].push(implicant);
    }

    // will look for combinations for current groups.
    // it will create a new groups matrix and replace the previous one.
    // when there are no more possible combinations to make, it will return false.
    fn combination_step(&mut self) -> bool {
        let mut new_groups = Vec::new();

        let mut old_groups_iterator = self.groups.iter().peekable();
        let mut found_some_combinable_this_step = false;

        while let Some(old_group) = old_groups_iterator.next() {
            let mut new_group = Vec::new();
            let next_group_option = old_groups_iterator.peek();
            if next_group_option.is_none() { break; }
            let next_group = *next_group_option.unwrap();

            for implicant in old_group.iter() {
                let mut found_combinable = false;

                for candidate in next_group.iter() {
                    if implicant.check_if_combines(candidate) {
                        found_combinable = true;
                        found_some_combinable_this_step = true;
                        let new_implicant = Implicant::from_implicants(implicant, candidate);

                        // after step 3 of iteration, the same implicant start to appear
                        // several times.
                        if ! new_group.contains(&new_implicant) {
                            new_group.push(new_implicant);
                        }
                    }
                }

                if ! found_combinable {
                    let mut prime_marked_clone = implicant.clone();
                    prime_marked_clone.mark_as_prime();
                    new_group.push(prime_marked_clone);
                }
            }

            new_groups.push(new_group);
        }

        // last group never combines, but must me kept.
        new_groups.push(self.groups.last().expect("Failed to load last group").clone());
        match new_groups.last_mut().expect("Failed to load last group").get_mut(0) {
            Some(implicant) => implicant.mark_as_prime(),
            None => {}
        }

        self.groups = new_groups;

        found_some_combinable_this_step
    }

    fn extract_primes(&self) -> Vec<&Implicant> {
        let mut primes = Vec::new();

        for group in &self.groups {
            for implicant in group {
                if ! implicant.is_prime() {
                    panic!("\
                    Found an implicant that is not marked as prime. This method should\
                    only be called after all combinations are made.\
                    ");
                }

                primes.push(implicant);
            }
        }

        primes
    }

    fn print_group(&self) {
        // 4 is the tabulation size, 3 is a right padding.
        let amount_of_variables = self.amount_of_groups - 1;
        let row_length = (amount_of_variables / 10 + 1) * 4 + self.amount_of_groups + 3;

        for group in 0..self.amount_of_groups {
            println!("{}", "-".repeat(row_length));
            print!("G{}", group);
            for implicant in &self.groups[group] {
                let prime_mark = if implicant.is_prime() { " *".to_string() } else { String::new() };
                println!("\t{}{}", implicant.get_binary_representation(), prime_mark);
            }
        }
        println!("{}", "-".repeat(row_length));
    }
}

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
            row.push(Cell::new(
                if *cover { "x" } else { " " }
            ));
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
