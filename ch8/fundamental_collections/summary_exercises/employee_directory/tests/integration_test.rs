extern crate employee_directory;

#[test]
fn insert_employee_retrieve_department_engineering() {
    let insert_string = "Add Sally to Engineering";
    let mut db = employee_directory::new();

    db.insert(insert_string);

    let employee_list = db.retrieve("Engineering");
    let expected_list = String::from("Dept: Engineering\n\tSally\n");
    assert_eq!(expected_list, employee_list);
}

#[test]
fn private_fields_are_private() {
    // won't compiletest_function_name
    // db.db.insert("hi".to_string(), vec!["hi".to_string()])
    // (although racer is happy to add completion)
}

#[test]
fn insert_two_employees_retrieve_engineering_alphabetical() {
    let insert_sally = "Add Sally to Engineering";
    let insert_roger = "Add Roger to Engineering";

    let mut db = employee_directory::new();

    db.insert(insert_sally);
    db.insert(insert_roger);

    let employee_list = db.retrieve("Engineering");
    let expected_list = String::from("Dept: Engineering\n\tRoger\n\tSally\n");
    assert_eq!(expected_list, employee_list);
}

#[test]
fn insert_and_retrieve_different_department() {
    let insert_bob = "Add Bob to Amazing";

    let mut db = employee_directory::new();

    db.insert(insert_bob);

    let expected_list = String::from("Dept: Amazing\n\tBob\n");
    let employee_list = db.retrieve("Amazing");
    assert_eq!(expected_list, employee_list);
}

#[test]
fn retrieve_all() {
    let insert_steph = "Add Steph to Operations";
    let insert_anne = "Add Anne to Corrections";

    let mut db = employee_directory::new();

    db.insert(insert_steph);
    db.insert(insert_anne);

    let employee_list = db.retrieve_all();
    let expected_list = String::from("Dept: Corrections\n\tAnne\nDept: Operations\n\tSteph\n");
    assert_eq!(expected_list, employee_list);
}

#[test]
fn retrieve_all_one_department() {
    let insert_steph = "Add Steph to Operations";

    let mut db = employee_directory::new();

    db.insert(insert_steph);

    let employee_list = db.retrieve_all();
    let expected_list = String::from("Dept: Operations\n\tSteph\n");
    assert_eq!(expected_list, employee_list);

}
