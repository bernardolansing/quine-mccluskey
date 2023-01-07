use crate::table_parser::TruthTable;

struct MintermFragment<'a> {
    from_table: &'a TruthTable,
    referring_variable: usize, // index of corresponding variable at from_table
    logic_load: bool // true -> A, false -> NOT A
}

impl MintermFragment<'_> {
    fn get_string_representation(self) -> String {
        let mut s = String::new();
        if ! self.logic_load {
            s.push_str("!");
        }

        s.push_str(self.from_table.get_header_at(self.referring_variable));

        s
    }
}

// struct Minterm<'a> {
//     referring_headers: Vec<usize>, // index of headers who this minterm refers to
//     from_table: &'a TruthTable
// }
//
// impl Minterm {
//     fn string_representation(self) -> String {
//         let mut s = String::new();
//
//         for h in self.referring_headers.iter() {
//             s.push_str(self.from_table.get_header_at(*h));
//         }
//
//         s
//     }
// }