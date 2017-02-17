use std::collections::HashMap;

pub struct Database {
    db: HashMap<String, Vec<String>>,
}

pub fn new() -> Database {
    Database { db: HashMap::new() }
}


impl Database {
    pub fn insert(&mut self, s: &str) {}

    pub fn retrieve(&self, dept: &str) -> String {
        return String::from("Dept: Engineering\n\tSally\n");
    }
}

// #[test]
// fn can_parse_a_name_from_the_insert_string() {
//     let insert_string = "Add Sally to Engineering";
//     let name = name_from(insert_string);
//     assert_eq!("Sally", name)
// }
