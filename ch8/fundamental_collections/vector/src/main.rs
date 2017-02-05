fn main() {
    // Does not compile!
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
}

#[test]
fn new_empty_vector() {
    let v: Vec<i32> = Vec::new();
    assert!(v.is_empty());
}

#[test]
fn instantiate_with_macro() {
    let vm = vec![1, 2, 3];
    assert_eq!(vm.len(), 3);
}

#[test]
fn push_it() {
    let mut vv = Vec::new();
    vv.push(5);
    vv.push(6);
    vv.push(7);
    vv.push(8);
    assert!(vv.len() == 4);
}

#[test]
fn reading() {
    let v = vec![1, 2, 3, 4, 5];
    // Subscript notation
    let third_i32: &i32 = &v[2];
    // Option baked in to the language
    let third_opt: Option<&i32> = v.get(2);
    assert_eq!(&3, third_i32);
    assert_eq!(Some(&3), third_opt);
}

#[test]
#[should_panic]
fn panic() {
    let v = vec![1, 2, 3, 4, 5];
    // panic! at the out of bounds exception disco
    &v[100];
}

#[test]
fn dont_panic() {
    let v = vec![1, 2, 3, 4, 5];
    // Option means never having to say panic!
    // at the out of bounds exception disco
    assert_eq!(None, v.get(100));
}

#[test]
fn multiple_types() {

    #[derive(Debug, PartialEq)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    assert_eq!(row[1], SpreadsheetCell::Text(String::from("blue")));
}
