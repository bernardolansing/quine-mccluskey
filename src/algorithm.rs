use crate::truth_table::TruthTable;
use crate::minterm::Implicant;

struct GroupStructure<'a> {
    amount_of_groups: usize,
    groups: Vec<Vec<&'a Implicant>>
}

impl<'a> GroupStructure<'a> {
    fn new(amount_of_variables: usize) -> Self {
        let mut groups: Vec<Vec<&Implicant>> = Vec::new();
        // +1 because a function with n variables can have n + 1 different amounts of true values
        (0..amount_of_variables + 1).for_each(|_| groups.push(Vec::new()));

        GroupStructure { groups, amount_of_groups: amount_of_variables + 1 }
    }

    fn add_implicant(&mut self, implicant: &'a Implicant) {
        let index = implicant.amount_of_true_variables();
        self.groups[index].push(implicant);
    }

    // in progress
    fn combination_step(&self) {
        for (index, group) in self.groups[0..self.groups.len() - 1].iter().enumerate() {
            for implicant in group {
                let mut already_found_combinable = false;

                for candidate in &self.groups[index + 1] {
                    let combines = implicant.check_if_combines(candidate);

                    if combines {
                        already_found_combinable = true;

                    }
                }
            }
        }
    }

    fn print_group(&self) {
        // 4 is the tabulation size, 3 is a right padding.
        let row_length = (self.amount_of_groups / 10 + 1) * 4 + self.amount_of_groups + 3;

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
    let amount_of_variables = table.amount_of_variables();
    let groups = agroup(&first_implicants, amount_of_variables);
    groups.print_group();
}

fn agroup(implicants: &Vec<Implicant>, amount_of_variables: usize) -> GroupStructure {
    let mut groups = GroupStructure::new(amount_of_variables);
    implicants.iter().for_each(|implicant| groups.add_implicant(implicant));

    groups
}

fn assemble_expression(implicants: &Vec<Implicant>) -> String {
    implicants.iter()
        .map(|implicant| implicant.get_string_representation())
        .collect::<Vec<String>>()
        .join(" + ")
}
