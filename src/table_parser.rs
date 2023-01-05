use csv::Reader;

pub struct TruthTable {
    headers: Vec<String>,
    values: Vec<Vec<bool>>
}

impl TruthTable {
    pub fn print_table(self) {
        self.headers.iter().for_each(|h| print!("{h}\t"));
        println!();
        self.values.iter().for_each(
            |row| {
                row.iter().for_each(|cell| print!("{} \t", *cell as i8));
                println!();
            }
        );
    }
}

pub fn read_csv(path: &str) -> TruthTable {
    let mut reader = Reader::from_path(path).expect("Error getting reader");

    let headers: Vec<String> = reader.headers().expect("Error getting headers")
        .iter().map(|s| String::from(s)).collect();

    let values: Vec<Vec<bool>> = reader.records()
        .map(
            |row| row.expect("Error while extracting row")
                .iter()
                .map(turn_input_into_boolean)
                .collect()
        ).collect();

    assert_ascending_order(&values);

    TruthTable { headers, values }
}

fn turn_input_into_boolean(c: &str) -> bool {
    if c.len() != 1 {
        panic!("Truth table cell values must be single character.");
    }

    let lower = c.to_lowercase();
    let comp = lower.as_str();

    match comp {
        "1" | "v" | "t" => true,
        "0" | "f" => false,
        other => panic!("Unknown character found: {}.", other)
    }
}

fn convert_boolean_row_to_number(row: &[bool]) -> usize {
    let mut weight: u32 = 0;
    let mut sum: usize = 0;

    for value in row.iter().rev() {
        if *value { sum += 2usize.pow(weight) }
        weight += 1;
    }

    sum
}

// check if inputs where provided in ascending order
fn assert_ascending_order(table_body: &Vec<Vec<bool>>) -> bool {
    for (expected_value, row) in table_body.iter().enumerate() {
        // we must discard the last item of each row, since that is the output
        let slice = &row[..row.len() - 1];
        if expected_value != convert_boolean_row_to_number(slice) {
            panic!("Inputs were not provided in ascending order or there are missing rows");
        }
    }

    true
}
