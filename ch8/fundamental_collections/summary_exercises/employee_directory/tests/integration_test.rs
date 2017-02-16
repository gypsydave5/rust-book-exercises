extern crate employee_directory;

#[test]
fn insert_employee_retrieve_department() {
    let insert_string = "Add Sally to Engineering";

    employee_directory::insert(insert_string);

    let expected_list = String::from("Dept: Engineering\n\tSally\n");
    let employee_list = employee_directory::retrieve("Engineering");
    assert_eq!(expected_list, employee_list);
}
