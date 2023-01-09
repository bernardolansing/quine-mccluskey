use crate::table_parser::read_csv;

pub struct TruthTable {
    variables: Vec<String>,
    inputs: Vec<Vec<bool>>,
    output: Vec<bool>
}

impl TruthTable {
    pub fn input_rows(&self) -> &Vec<Vec<bool>> { &self.inputs }
    pub fn variables_names(&self) -> Vec<String> { self.variables.clone() }
    pub fn amount_of_variables(&self) -> usize {
        self.inputs.get(0).unwrap_or(&Vec::new() as &Vec<bool>).len()
    }
    pub fn row_value(&self, index: usize) -> bool {
        *self.output.get(index).expect("Not a valid row index")
    }

    pub fn from_csv(path: &str) -> Self {
        let (variables, inputs, output) = read_csv(path);
        TruthTable { variables, inputs, output }
    }

    pub fn print_table(&self) {
        self.variables.iter().for_each(|h| print!("{h}\t"));
        println!();
        self.inputs.iter().enumerate().for_each(
            |(index, row)| {
                row.iter().for_each(|cell| print!("{} \t", *cell as i8));
                print!("{}", self.output[index] as i8);
                println!();
            }
        );
    }

    pub fn get_header_at(&self, index: usize) -> &String {
        match self.variables.get(index) {
            Some(s) => s,
            None => panic!(
                "Failed to access table header. Tried to get [{index}] but size is {}",
                self.variables.len()
            )
        }
    }
}