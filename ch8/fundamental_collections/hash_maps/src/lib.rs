use std::collections::HashMap;

#[test]
fn inserting_into_a_hash_map() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("blue team"), 5);
    scores.insert(String::from("red team"), 2);

    assert_eq!(Some(&5), scores.get("blue team"));
}

#[test]
fn building_with_collect() {}
