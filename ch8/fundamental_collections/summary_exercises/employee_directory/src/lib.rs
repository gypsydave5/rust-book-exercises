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
        let d = self.get_department(dept);
        d.to_string() + "\n"
    }

    pub fn retrieve_all(&self) -> String {
        let mut x: Vec<String> = self.db
            .iter()
            .map(|(d, ns)| [&department_header(&d)[..], &ns.join("\n\t")[..]].join("\n\t"))
            .collect();

        x.sort();
        x.join("\n") + "\n"
    }

    fn insert_employee(&mut self, name: &str, department: &str) {
        let mut dep = self.db.entry(department.to_string()).or_insert(Vec::new());
        dep.push(String::from(name));
        dep.sort();
    }

    fn get_department(&self, dept: &str) -> Department {
        Department {
            name: dept.to_string(),
            employees: self.db.get(dept).unwrap().clone(),
        }
    }
}

fn department_header(d: &str) -> String {
    ["Dept: ", d].concat().clone()
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

struct Department {
    name: String,
    employees: Vec<String>,
}

impl Department {
    fn to_string(&self) -> String {
        [department_header(&self.name), self.employees.join("\n\t")].join("\n\t")
    }
}

#[test]
fn department_to_string() {
    let d = Department {
        name: "Operations".to_string(),
        employees: vec!["Steph".to_string(), "Dave".to_string()],
    };

    assert_eq!(d.to_string(), "Dept: Operations\n\tSteph\n\tDave")
}
