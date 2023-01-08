use crate::truth_table::TruthTable;
use csv::Reader;

pub fn read_csv(path: &str) -> (Vec<String>, Vec<Vec<bool>>, Vec<bool>) {
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

    let inputs: Vec<Vec<bool>> = values.iter().map(
        |row| row[..row.len() - 1].to_vec()
    ).collect();

    let output = values.iter().map(
        |row| *row.last().expect("Error while extracting output")
    ).collect();

    assert_ascending_order(&inputs);

    (headers, inputs, output)
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
fn assert_ascending_order(inputs: &Vec<Vec<bool>>) {
    for (expected_value, row) in inputs.iter().enumerate() {
        if expected_value != convert_boolean_row_to_number(row) {
            panic!("Inputs were not provided in ascending order or there are missing rows");
        }
    }
}
