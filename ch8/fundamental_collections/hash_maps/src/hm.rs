use std::collections::HashMap;

#[test]
fn inserting_into_a_hash_map() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("blue team"), 5);
    scores.insert(String::from("red team"), 2);

    assert_eq!(Some(&5), scores.get("blue team"));
}

#[test]
fn building_with_collect() {
    let s = scores();
    assert_eq!(&2, s.get("red").unwrap())
}

#[test]
fn moving_immutable() {
    let mut s2 = scores();
    // need mut to make this work
    {
        let x: &mut i32 = s2.get_mut("blue").unwrap();
        // type annotation to show what's what - it's an &mut - a mutable reference
        *x += 5;
        // dereference and mutate!
    }

    assert_eq!(Some(&10), s2.get("blue"))
}

// OK, this is interesting...

pub fn scores() -> HashMap<String, i32> {
    let teams: Vec<String> = vec!["blue".to_string(), "red".to_string()];
    let initial_scores: Vec<i32> = vec![5, 2];

    // Rust has three flavours of iterator

    let s1: HashMap<&String, &i32> = teams.iter()
        .zip(initial_scores.iter())
        .collect();
    // .iter() borrows

    let s3: HashMap<String, i32> = teams.clone()
        .into_iter()
        .zip(initial_scores.clone().into_iter())
        .collect();
    // .into_iter() copies
    // and we need to copy as we cannot return a copy of something scoped to the function (s1 is
    // a HashMap of references - of borrows); we need a brand new thing.

    // there's also .mut_iter();
    s3
}
