use prettytable::{Table, Row, Cell, Attr};
use crate::implicant::Implicant;

const SELECTED_PRIME_COLOR: u32 = 0x10AA10;

pub struct CoverageMap {
    map: Vec<Vec<bool>>,
    selected_primes: Vec<usize>,
    primes_names: Vec<String>,
    minterms_names: Vec<String>
}

impl CoverageMap {
    pub fn new(prime_implicants: &Vec<&Implicant>, basic_implicants: &Vec<Implicant>) -> Self {
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

        CoverageMap { map, primes_names, minterms_names, selected_primes }
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
                            self.selected_primes.push(prime_index);
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
                let mut new_cell = Cell::new(if *cover { "X" } else { " " })
                    .style_spec("c")
                    .style_spec(row_spec);
                new_row.push(new_cell);
            }

            table.add_row(Row::new(new_row));
        }

        table.printstd();
    }
}
