#[test]
fn create_empty_string() {
    let s = String::new();
    assert_eq!(s, "");
}

#[test]
fn initialize_string_with_to_string() {
    let data = "string";
    let s = data.to_string();
    assert_eq!(s, "string");

    let s = "one line".to_string();
    assert_eq!(s, "one line");
}

#[test]
fn initialize_string_with_from() {}
