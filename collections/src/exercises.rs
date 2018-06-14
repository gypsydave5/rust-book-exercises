use std::collections::HashMap;

pub struct AverageResult {
    pub mode: u32,
    pub median: u32,
    pub mean: u32,
}

pub fn average(numbers: Vec<u32>) -> AverageResult {
    let halfway = numbers.len() / 2;
    let mut sorted = numbers.clone();
    sorted.sort();
    let median = sorted.get(halfway).unwrap().clone();

    let mut map = HashMap::new();
    for number in numbers.clone().into_iter() {
        let number_count = map.entry(number).or_insert(0);
        *number_count += 1;
    }
    let mode = map.into_iter().max_by_key(|x| x.1).unwrap().0;

    let sum: u32 = numbers.clone().into_iter().sum();
    let mean: u32 = sum / (numbers.len() as u32);

    AverageResult {
        mode: mode,
        median: median,
        mean: mean,
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

    #[test]
    fn it_returns_the_mean() {
        let numbers = vec![2, 4, 5, 1, 3, 3];
        let mean = average(numbers).mean;
        assert_eq!(3, mean);
    }
}
