use crate::table_parser::convert_boolean_row_to_number;

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

#[derive(Clone)]
struct MintermFragment {
    variable_name: String,
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

#[derive(Clone)]
pub struct Implicant {
    variables_names: Vec<String>,
    fragments: Vec<MintermFragment>,
    marked_as_prime: bool
}

impl Implicant {
    pub fn is_prime(&self) -> bool { self.marked_as_prime }
    pub fn mark_as_prime(&mut self) { self.marked_as_prime = true; }

    // creates the first implicants, which are minterms built from input rows.
    pub fn from_input(row_of_inputs: &Vec<bool>, variables_names: Vec<String>) -> Self {
        let mut fragments = Vec::new();

        for (index, value) in row_of_inputs.iter().enumerate() {
            let logic_load = if *value { LogicLoad::True } else { LogicLoad::False };
            fragments.push(
                MintermFragment {
                    variable_name: variables_names[index].to_string(),
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

    pub fn clone(&self) -> Self {
        Self {
            variables_names: self.variables_names.clone(),
            fragments: self.fragments.clone(),
            marked_as_prime: false,
        }
    }

    pub fn amount_of_true_variables(&self) -> usize {
        self.fragments.iter()
            .filter(|frag| match frag.logic_load { LogicLoad::True => true, _ => false })
            .count()
    }

    // for implicants that are minterms, this method returns its associated number
    // this number is the index of its corresponding row on the truth table (if it starts from
    // least significative inputs towards the most significatives ones)
    pub fn minterm_number(&self) -> usize {
        let mut logic_loads = Vec::new();

        for variable in &self.fragments {
            match variable.logic_load {
                LogicLoad::True => logic_loads.push(true),
                LogicLoad::False => logic_loads.push(false),
                LogicLoad::DontMatter => panic!(
                    "This implicant is not equivalent to a minterm.\
                    This method can only be called over minterms."
                )
            }
        }

        convert_boolean_row_to_number(logic_loads.as_slice())
    }

    pub fn check_if_combines(&self, other: &Implicant) -> bool {
        if self.marked_as_prime || other.marked_as_prime { return false; }

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

    // check if other implicant may be logically covered by self.
    pub fn covers(&self, other: &Implicant) -> bool {
        for (self_var, other_var) in self.fragments.iter().zip(other.fragments.iter()) {
            let this_matches = match (self_var.logic_load, other_var.logic_load) {
                (LogicLoad::True, LogicLoad::False) => false,
                (LogicLoad::False, LogicLoad::True) => false,
                _ => true
            };

            if ! this_matches { return false }
        }

        true
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

impl PartialEq for Implicant {
    fn eq(&self, other: &Self) -> bool {
        for (self_var, other_var) in self.fragments.iter().zip(other.fragments.iter()) {
            if ! self_var.logic_load.equals(&other_var.logic_load) {
                return false;
            }
        }

        true
    }
}
