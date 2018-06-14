use std::collections::HashMap;

pub struct AverageResult {
    pub mode: u32,
    pub median: u32,
}

pub fn average(numbers: Vec<u32>) -> AverageResult {
    let mut map = HashMap::new();
    let halfway = numbers.len() / 2;
    let mut sorted = numbers.clone();
    sorted.sort();
    let median = sorted.get(halfway).unwrap().clone();

    for number in numbers.into_iter() {
        let number_count = map.entry(number).or_insert(0);
        *number_count += 1;
    }

    let mode = map.into_iter().max_by_key(|x| x.1).unwrap().0;

    AverageResult {
        mode: mode,
        median: median,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_mode() {
        let numbers = vec![1, 2, 3, 3, 3, 4, 5, 6];
        let mode = average(numbers).mode;
        assert_eq!(3, mode);
    }

    #[test]
    fn it_returns_the_median() {
        let numbers = vec![4, 1, 1, 10, 10];
        let median = average(numbers).median;
        assert_eq!(4, median);
    }
}
