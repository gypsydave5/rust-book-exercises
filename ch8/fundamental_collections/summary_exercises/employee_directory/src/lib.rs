use std::collections::HashMap;

pub struct Database {
    db: Vec<String>,
}

pub fn new() -> Database {
    Database { db: Vec::new() }
}

impl Database {
    pub fn insert(&mut self, s: &str) {
        let name = name_from(s);
        self.db.push(String::from(name));
    }

    pub fn retrieve(&self, dept: &str) -> String {
        let header = "Dept: Engineering";
        let names = self.db.join("\n\t");
        [header, &names].join("\n\t") + "\n"
    }
}

fn name_from(s: &str) -> &str {
    s.split(" ").nth(1).unwrap()
}

#[test]
fn can_parse_a_name_from_the_insert_string() {
    let insert_string = "Add Sally to Engineering";
    let name = name_from(insert_string);
    assert_eq!("Sally", name)
}
