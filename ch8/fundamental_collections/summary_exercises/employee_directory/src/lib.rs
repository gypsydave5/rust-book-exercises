use std::collections::HashMap;

pub struct Database {
    db: HashMap<String, Vec<String>>,
}

pub fn new() -> Database {
    Database { db: HashMap::new() }
}

impl Database {
    pub fn insert(&mut self, s: &str) {
        let name = name_from(s);
        let department = department_from(s);
        self.insert_employee(name, department);
    }

    pub fn retrieve(&mut self, dept: &str) -> String {
        let header = ["Dept: ", dept].concat();
        let names = self.get_names_list(dept);
        [header, names].join("\n\t") + "\n"
    }

    fn get_names_list(&mut self, dept: &str) -> String {
        let mut names = self.db.get_mut(dept).unwrap();
        names.sort();
        names.join("\n\t")
    }

    fn insert_employee(&mut self, name: &str, department: &str) {
        let mut dep = self.db.entry(department.to_string()).or_insert(Vec::new());
        dep.push(String::from(name));
    }
}

fn name_from(s: &str) -> &str {
    s.split(" ").nth(1).unwrap()
}

fn department_from(s: &str) -> &str {
    s.split(" ").nth(3).unwrap()
}

#[test]
fn can_parse_a_name_from_the_insert_string() {
    let insert_string = "Add Sally to Engineering";
    let name = name_from(insert_string);
    assert_eq!("Sally", name)
}

#[test]
fn can_parse_a_department_from_the_insert_string() {
    let insert_string = "Add Bob to Amazing";
    let department = department_from(insert_string);
    assert_eq!("Amazing", department)
}
