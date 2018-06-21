mod average;
mod pig_latin;

pub fn average(numbers: &Vec<u32>) -> average::AverageResult {
    average::average(numbers)
}

pub fn pig_latin(sentence: String) -> String {
    pig_latin::pig_latin(sentence)
}

