use crate::implicant::Implicant;

pub struct GroupStructure {
    amount_of_groups: usize,
    groups: Vec<Vec<Implicant>>
}

impl GroupStructure {
    pub fn new(amount_of_variables: usize) -> Self {
        let mut groups: Vec<Vec<Implicant>> = Vec::new();
        // +1 because a function with n variables can have n + 1 different amounts of true values
        (0..amount_of_variables + 1).for_each(|_| groups.push(Vec::new()));

        GroupStructure { groups, amount_of_groups: amount_of_variables + 1 }
    }

    pub fn add_implicant(&mut self, implicant: Implicant) {
        let index = implicant.amount_of_true_variables();
        self.groups[index].push(implicant);
    }

    // will look for combinations for current groups.
    // it will create a new groups matrix and replace the previous one.
    // when there are no more possible combinations to make, it will return false.
    pub fn combination_step(&mut self) -> bool {
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

    pub fn extract_primes(&mut self) -> Vec<Implicant> {
        let mut primes = Vec::new();

        for group in self.groups.iter_mut() {
            for index in (0..group.len()).into_iter().rev() {
                if ! group[index].is_prime() {
                    panic!("\
                    Found an implicant that is not marked as prime. This method should\
                    only be called after all combinations are made.\
                    ");
                }

                primes.push(group.remove(index));
            }
        }

        primes
    }

    pub fn print_group(&self) {
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