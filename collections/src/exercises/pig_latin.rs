pub fn pig_latin(sentence: String) -> String {
    sentence
        .split_whitespace()
        .map(|word| pig_latin_word(word))
        .fold(String::new(), |acc, word| if acc.len() == 0 {
            acc + &word
        } else {
            acc + " " + &word
        })
}

fn pig_latin_word(word: &str) -> String {
    let mut iter = word.chars().peekable();
    let mut pre_punctuation = String::new();
    let mut word_without_punctuation = String::new();

    while iter.peek()
        .map(|c| c.is_ascii_punctuation())
        .unwrap_or_default()
    {
        pre_punctuation.push(iter.next().unwrap())
    }

    while iter.peek()
        .map(|c| !c.is_ascii_punctuation())
        .unwrap_or_default()
    {
        word_without_punctuation.push(iter.next().unwrap())
    }

    let post_punctuation: String = iter.collect();

    pre_punctuation + &pig_latin_unpunctuated_word(&word_without_punctuation) + &post_punctuation
}

fn pig_latin_unpunctuated_word(word: &str) -> String {
    let first_letter = &word[0..1];
    let vowels = "aeiou";
    let mut s = String::new();

    if vowels.contains(first_letter) {
        s.push_str(&word);
        s.push_str("-hay");
    } else {
        s.push_str(&word[1..]);
        s.push_str("-");
        s.push_str(&word[0..1]);
        s.push_str("ay");
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pig_latin_simple_words() {
        let original_string = String::from("hello world");
        let expected = String::from("ello-hay orld-way");
        pig_test(original_string, expected)
    }

    #[test]
    fn pig_latin_words_starting_with_vowels() {
        let original_string = String::from("orange apples");
        let expected = String::from("orange-hay apples-hay");
        pig_test(original_string, expected)
    }

    #[test]
    fn pig_latin_with_punctuation() {
        let original_string = String::from("hello, world");
        let expected = String::from("ello-hay, orld-way");
        pig_test(original_string, expected)
    }

    #[test]
    fn pig_latin_excited_spanish_punctuation() {
        let original_string = String::from("!!hola!!");
        let expected = String::from("!!ola-hay!!");
        pig_test(original_string, expected)
    }

    fn pig_test(sentence: String, expected: String) {
        let actual = pig_latin(sentence);
        assert_eq!(expected, actual)
    }
}
