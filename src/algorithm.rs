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

        GroupStructure { groups, amount_of_groups: amount_of_variables }
    }

    fn add_implicant(&mut self, implicant: &'a Implicant) {
        let index = implicant.amount_of_true_variables();
        self.groups[index].push(implicant);
    }

    fn print_group(&self) {
        // 4 is the tabulation size, 3 is a right padding.
        let row_length = (self.amount_of_groups / 10 + 1) * 4 + self.amount_of_groups + 3;

        for group in 0..self.amount_of_groups {
            println!("{}", "-".repeat(row_length));
            print!("G{}", group);
            self.groups[group].iter().for_each(|g| println!("\t{}", g.get_binary_representation()));
        }
        println!("{}", "-".repeat(row_length));
    }
}

pub fn algorithm(table: TruthTable) {
    let mut first_implicants: Vec<Implicant> = Vec::new();
    for (index, row) in table.input_rows().iter().enumerate() {
        if table.row_value(index) {
            first_implicants.push(Implicant::new(row, table.variables_names()));
        }
    }

    println!("Function is defined by the unoptimized expression:");
    println!("{}", first_implicants.iter()
        .map(|implicant| implicant.get_string_representation())
        .collect::<Vec<String>>()
        .join(" + ")
    );

    println!("\nBeggining iterative optimization by Quine-McCluskey algorithm.");
    let amount_of_groups = table.amount_of_variables();
    let groups = agroup(&first_implicants, amount_of_groups);
    groups.print_group();
}

fn agroup(implicants: &Vec<Implicant>, amount_of_groups: usize) -> GroupStructure {
    let mut groups = GroupStructure::new(amount_of_groups);
    implicants.iter().for_each(|implicant| groups.add_implicant(implicant));

    groups
}
