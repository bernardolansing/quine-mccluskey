use crate::truth_table::TruthTable;

enum LogicLoad {
    False,
    True,
    DontMatter
}

struct MintermFragment {
    variable_name: String,
    // referring_variable: usize, // index of corresponding variable at from_table
    logic_load: LogicLoad
}

impl MintermFragment {
    fn get_string_representation(&self) -> String {
        match self.logic_load {
            LogicLoad::False => format!("!{}", self.variable_name),
            LogicLoad::DontMatter => String::new(),
            LogicLoad::True => self.variable_name.to_string()
        }
    }
}

pub struct Implicant {
    variables_names: Vec<String>,
    fragments: Vec<MintermFragment>
}

impl Implicant {
    // this overload creates the first implicants, which are built from input rows.
    pub fn new(row_of_inputs: &Vec<bool>, variables_names: Vec<String>) -> Self {
        let mut fragments = Vec::new();

        for (index, value) in row_of_inputs.iter().enumerate() {
            let logic_load = if *value { LogicLoad::True } else { LogicLoad::False };
            fragments.push(
                MintermFragment {
                    variable_name: variables_names[index].to_string(),
                    // referring_variable: index,
                    logic_load
                }
            );
        }

        Implicant { variables_names, fragments }
    }

    pub fn get_string_representation(&self) -> String {
        let mut rep = String::new();

        for frag in &self.fragments {
            rep.push_str(frag.get_string_representation().as_str());
        }

        rep
    }
}
