use std::collections::HashMap;
use std::iter::FromIterator;
use std;

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
        // also need another scope in order to do the immutable borrow below
        let x: &mut i32 = s2.get_mut("blue").unwrap();
        // type annotation to show what's what - it's an &mut - a mutable reference
        *x += 5;
        // dereference and mutate!
    }

    assert_eq!(Some(&10), s2.get("blue"))
}

#[test]
fn copy_or_move() {
    let key = String::from("key");
    let value = 50;

    let mut map = HashMap::new();
    map.insert(key, value);
    assert_eq!(value, 50);
    // assert_eq!(key, "hi");
    // This will not compile as Strings do not implement Copy(), unlike i32s. So the value is
    // a copy, but he key has been moved into map - so the old reference is gone.
}

#[test]
fn overwriting_a_value() {
    let mut map = HashMap::new();

    map.insert("Blue", 5);
    assert_eq!(Some(&5), map.get("Blue"));

    map.insert("Blue", 10);
    assert_eq!(Some(&10), map.get("Blue"));
}

#[test]
fn insert_if_empty() {
    let mut map: HashMap<_, _> = std::iter::repeat(("key", 1))
        .take(1)
        .collect();

    map.entry("key").or_insert(5);
    map.entry("new_key").or_insert(10);

    assert_eq!(Some(&1), map.get("key"));
    assert_eq!(Some(&10), map.get("new_key"));
}

#[test]
fn update_based_on_old() {
    let text = "hello world you wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    assert_eq!(Some(&2), map.get("world"));
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
