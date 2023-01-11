use prettytable::{Table, Row, Cell};
use crate::implicant::Implicant;

pub struct CoverageMap {
    map: Vec<Vec<bool>>,
    selected_primes: Vec<usize>,
    primes_names: Vec<String>,
    minterms_names: Vec<String>,
    covered_minterms: Vec<usize>
}

impl CoverageMap {
    pub fn new(prime_implicants: &Vec<Implicant>, basic_implicants: &Vec<Implicant>) -> Self {
        let mut map = Vec::new();

        let primes_names: Vec<String> = prime_implicants
            .iter()
            .map(|p| p.get_string_representation())
            .collect();
        let minterms_names: Vec<String> = basic_implicants
            .iter()
            .map(|m| format!("m{}", m.minterm_number()))
            .collect();

        for prime in prime_implicants {
            let mut row = Vec::new();
            for minterm in basic_implicants {
                row.push(prime.covers(minterm));
            }
            map.push(row);
        }

        let selected_primes = Vec::new();
        let covered_minterms = Vec::new();

        CoverageMap { map, primes_names, minterms_names, selected_primes, covered_minterms }
    }

    // if some prime is the only who covers certain minterm, this prime is essential.
    pub fn find_essentials(&mut self) -> usize {
        let mut essentials_found: usize = 0;

        for column in 0..self.minterms_names.len() {
            let mut covering_prime: Option<usize> = None;
            let mut found_multiple_covering_primes = false;

            for row in 0..self.primes_names.len() {
                if self.map[row][column] {
                    // had previously found another covering prime, so there is no
                    // essential prime for this minterm.
                    if covering_prime.is_some() { found_multiple_covering_primes = true }
                    else { covering_prime = Some(row) }
                }
            }

            if ! found_multiple_covering_primes {
                match covering_prime {
                    Some(prime_index) => {
                        if ! self.selected_primes.contains(&prime_index) {
                            self.select_implicant(prime_index);
                            essentials_found += 1;
                        }
                    },
                    None => panic!(
                        "Found no covering prime for {}, despite all minterms should be\
                        covered at this point.", self.minterms_names[column]
                    )
                }
            }
        }

        essentials_found
    }

    fn select_implicant(&mut self, index: usize) {
        if self.selected_primes.contains(&index) { return }
        self.selected_primes.push(index);

        let minterms_covered: Vec<usize> = (0..self.map[index].len())
            .filter(|minterm_index| self.map[index][*minterm_index])
            .collect();

        for covered in minterms_covered {
            if ! self.covered_minterms.contains(&covered) {
                self.covered_minterms.push(covered);
            }
        }
    }

    fn covers_how_many_uncovered(&self, implicant_index: usize) -> usize {
        let mut covered_by_this_implicant = Vec::new();
        &self.map[implicant_index].iter()
            .enumerate()
            .for_each(|(index, covers)| if *covers { covered_by_this_implicant.push(index) });

        covered_by_this_implicant.iter()
            .filter(|covered| ! self.covered_minterms.contains(covered))
            .count()
    }

    // equals to the amount
    fn how_many_significative_variables(&self, implicant_index: usize) -> usize {
        let implicant_name = &self.primes_names[implicant_index];
        implicant_name.chars().filter(|c| *c != '-').count()
    }

    // after running find_essentials, we can now choose which of the remaining primes will
    // be selected. for each uncovered minterm, we will choose one of the primes that covers
    // more yet uncovered minterms.
    pub fn choose_remaining_primes(&mut self) {
        for minterm_index in 0..self.minterms_names.len() {
            if self.covered_minterms.contains(&minterm_index) { continue }

            // implicants that cover the yet uncovered minterm
            let prime_candidates_indexes: Vec<usize> = (0..self.primes_names.len())
                .filter(|index| self.map[*index][minterm_index])
                .collect();

            // we will want to select the candidate who covers the largest number of yet
            // uncovered implicants. if many options are available, we will just
            // select the first one.

            let candidates_and_new_covertures: Vec<(usize, usize)> = prime_candidates_indexes.iter()
                .map(|prime_index| (*prime_index, self.covers_how_many_uncovered(*prime_index)))
                .collect();

            let max_new_covertures = candidates_and_new_covertures.iter()
                .map(|(_, new_cov)| new_cov)
                .max()
                .expect("Failed to find implicant that covers more uncovered minterms.");

            let candidate_to_be_selected: usize = candidates_and_new_covertures.iter()
                .find(|(_, cand_new_cov)| *cand_new_cov == *max_new_covertures)
                .expect("Failed to find implicant who had the max new covers.")
                .0;

            self.select_implicant(candidate_to_be_selected);
        }
    }

    pub fn get_selected_implicants(&self) -> &Vec<usize> { &self.selected_primes }

    pub fn print(&self) {
        let mut table = Table::new();

        let header_vec: Vec<String> = vec![String::new()]
            .iter()
            .chain(self.minterms_names.iter())
            .cloned()
            .collect();
        let header_row = Row::new(
            // style_spec("c") sets alignment to center
            header_vec.iter().map(|c| Cell::new(c.as_str()).style_spec("c")).collect()
        );
        table.add_row(header_row);

        for (row_index, prime_name) in self.primes_names.iter().enumerate() {
            let mut new_row = Vec::new();
            // if this minterm is selected, it will be printed in green
            let row_spec = if self.selected_primes.contains(&row_index) { "Fg" } else { "" };
            new_row.push(Cell::new(prime_name.as_str()).style_spec(row_spec));

            for cover in &self.map[row_index] {
                let new_cell = Cell::new(if *cover { "X" } else { " " })
                    .style_spec("c")
                    .style_spec(row_spec);
                new_row.push(new_cell);
            }

            table.add_row(Row::new(new_row));
        }

        table.printstd();
    }
}
