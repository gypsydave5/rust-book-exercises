use std::collections::HashMap;

fn main() {}

fn averages(xs: &Vec<i32>) -> (f32, i32, i32) {
    let mean = mean(xs);
    let median = median(xs);
    let mode = mode(xs);
    return (mean, median, mode);
}

fn mean(xs: &Vec<i32>) -> f32 {
    let sum: i32 = xs.into_iter().sum();
    let length = xs.len() as f32;

    return sum as f32 / length;
}

fn mode(xs: &Vec<i32>) -> i32 {
    let mut m = HashMap::new();
    for x in xs {
        let v = m.entry(x).or_insert(0);
        *v += 1;
    }
    let (v, _) = m.iter()
        .max_by_key(|&(_, count)| count)
        .unwrap();
    return v.to_owned().to_owned();
}

fn median(xs: &Vec<i32>) -> i32 {
    let mut ordered = xs.to_owned();
    ordered.sort();
    return ordered[ordered.len() / 2];
}

#[test]
fn test_averages() {
    let (mean, median, mode) = averages(&vec![2, 2, 1, 4, 6]);
    assert_eq!(mean, 3.0);
    assert_eq!(median, 2);
    assert_eq!(mode, 2);
}

#[test]
fn mean_of_one_value() {
    let v = vec![5];
    assert_eq!(5.0, mean(&v));
}

#[test]
fn mean_of_two_values() {
    let v = vec![2, 3];
    assert_eq!(2.5, mean(&v));
}

#[test]
fn mode_of_one_value() {
    let v = vec![5];
    assert_eq!(5, mode(&v));
}

#[test]
fn mode_of_three_values() {
    let v = vec![5, 1, 1];
    assert_eq!(1, mode(&v));
}

#[test]
fn median_of_one_value() {
    let v = vec![3];
    assert_eq!(3, median(&v));
}

#[test]
fn median_of_three_values() {
    let v = vec![5, 3, 4, 2, 1];
    assert_eq!(3, median(&v));
}
