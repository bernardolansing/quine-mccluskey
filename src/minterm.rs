use std::mem::discriminant;
use crate::truth_table::TruthTable;

#[derive(Copy, Clone)]
enum LogicLoad { False, True, DontMatter }

impl LogicLoad {
    fn is_true(&self) -> bool { match self { Self::True => true, _ => false } }
    fn dont_matter(&self) -> bool { match self { Self::DontMatter => true, _ => false } }
    fn is_false(&self) -> bool { match self { Self::False => true, _ => false } }
    fn equals(&self, other: &LogicLoad) -> bool {
        let both_true = self.is_true() && other.is_true();
        let both_dont_matter = self.dont_matter() && other.dont_matter();
        let both_false = self.is_false() && other.is_false();
        both_true || both_dont_matter || both_false
    }
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

    fn get_binary_representation(&self) -> String {
        match self.logic_load {
            LogicLoad::False => String::from("0"),
            LogicLoad::DontMatter => String::from("-"),
            LogicLoad::True => String::from("1")
        }
    }
}

pub struct Implicant {
    variables_names: Vec<String>,
    fragments: Vec<MintermFragment>,
    marked_as_prime: bool // a priori, will be set to false. Algorithm will set it to true
    // if later verified that it is a prime.
}

impl Implicant {
    pub fn is_prime(&self) -> bool { self.marked_as_prime }
    pub fn mark_as_prime(&mut self) { self.marked_as_prime = true }

    // creates the first implicants, which are built from input rows.
    pub fn from_input(row_of_inputs: &Vec<bool>, variables_names: Vec<String>) -> Self {
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

        Implicant { variables_names, fragments, marked_as_prime: false }
    }

    pub fn from_implicants(impl_a: &Implicant, impl_b: &Implicant) -> Self {
        let mut new_fragments = Vec::new();
        let mut unequal_fragments_found: usize = 0;

        for (frag_a, frag_b) in impl_a.fragments.iter().zip(impl_b.fragments.iter()) {
            let variable_name = frag_a.variable_name.to_string();

            if frag_a.logic_load.equals(&frag_b.logic_load) {
                let logic_load = frag_a.logic_load;
                new_fragments.push(MintermFragment { variable_name, logic_load });
            }

            else {
                unequal_fragments_found += 1;
                new_fragments.push(
                    MintermFragment { variable_name, logic_load: LogicLoad::DontMatter }
                );
            }
        }

        if unequal_fragments_found != 1 { panic!("Trying to combine incompatible implicants.") }

        Implicant {
            variables_names: impl_a.variables_names.clone(),
            fragments: new_fragments,
            marked_as_prime: false,
        }
    }

    pub fn amount_of_true_variables(&self) -> usize {
        self.fragments.iter()
            .filter(|frag| match frag.logic_load { LogicLoad::True => true, _ => false})
            .count()
    }

    pub fn check_if_combines(&self, other: &Implicant) -> bool {
        let mut differences: usize = 0;
        for (self_var, other_var) in self.fragments.iter().zip(other.fragments.iter()) {
            match (&self_var.logic_load, &other_var.logic_load) {
                (LogicLoad::True, LogicLoad::True) => {},
                (LogicLoad::DontMatter, LogicLoad::DontMatter) => {},
                (LogicLoad::False, LogicLoad::False) => {},
                _ => { differences += 1; }
            }
        }

        differences == 1
    }

    pub fn get_string_representation(&self) -> String {
        let mut rep = String::new();

        for frag in &self.fragments {
            rep.push_str(frag.get_string_representation().as_str());
        }

        rep
    }

    pub fn get_binary_representation(&self) -> String {
        let mut rep = String::new();

        for frag in &self.fragments {
            rep.push_str(frag.get_binary_representation().as_str());
        }

        rep
    }
}
