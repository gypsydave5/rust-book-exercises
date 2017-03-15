#[test]
fn simple_iterating() {
    let v1 = vec![1,2,3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, [2,3,4]);
}

struct Counter {
    count: i32,
}
