extern crate employee_directory;

#[test]
fn insert_employee_retrieve_department() {
    let insert_string = "Add Sally to Engineering";
    let mut db = employee_directory::new();

    db.insert(insert_string);

    let expected_list = String::from("Dept: Engineering\n\tSally\n");
    let employee_list = db.retrieve("Engineering");
    assert_eq!(expected_list, employee_list);
}

#[test]
fn private_fields_are_private() {
    // won't compiletest_function_name
    // db.db.insert("hi".to_string(), vec!["hi".to_string()])
    // (although racer is happy to add completion)
    // (but not for the public methods...)
}

#[test]
fn insert_two_employees_retrieve_department_alphabetical() {
    let insert_sally = "Add Sally to Engineering";
    let insert_roger = "Add Roger to Engineering";

    let mut db = employee_directory::new();

    db.insert(insert_sally);
    db.insert(insert_roger);

    let expected_list = String::from("Dept: Engineering\n\tRoger\n\tSally\n");
    let employee_list = db.retrieve("Engineering");
    assert_eq!(expected_list, employee_list);
}
