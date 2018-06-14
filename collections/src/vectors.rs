pub fn accessing_elements() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let third = &v[2];

    let v_index = 2;
    match v.get(v_index) {
        Some(n) => println!("Reached {} at index {}", n, v_index),
        None => println!("Unable to reach element at index {}", v_index),
    };

    v.get(v_index).map(|n| println!("{}", n));
}

pub fn iteration() {
    let v = vec![4, 8, 15];
    for i in &v {
        println!("{}", i);
    }
}

pub fn mutable_iteration() {
    let mut v = vec![4, 8, 15];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
}

fn no_mutation_of_vec_with_borrowed_elements() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    //    v.push(6);
}

fn enum_vector_hack() {
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
}
