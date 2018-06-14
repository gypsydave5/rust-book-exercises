use std::collections::HashMap;

pub fn blue_vs_yellow() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores.insert(String::from("Blue"), 25);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // into_iter consumes; iter borrows
    let iter = teams.into_iter().zip(initial_scores.into_iter());
    let other_iter = iter.clone();

    // can collect from an iterator into different collections
    // depending upon type annotation...
    let mut scores: HashMap<String, u32> = iter.collect();
    let vector: Vec<_> = other_iter.collect();

    println!("Scores: {:?}", scores);
    println!("Vector: {:?}", vector);

    // overwriting
    scores.insert(String::from("Blue"), 25);
    println!("Scores: {:?}", scores);

    // insert if absent
    scores.entry(String::from("Green")).or_insert(111);
    scores.entry(String::from("Blue")).or_insert(1337);
    println!("Scores: {:?}", scores);
}

pub fn upsert() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Word count: {:?}", map);
}
