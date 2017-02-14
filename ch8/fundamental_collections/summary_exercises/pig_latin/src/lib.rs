pub fn pig_latin(word: String) -> String {
    let first_letter = word.chars().nth(0).unwrap();

    match first_letter {
        'a' | 'e' | 'i' | 'o' | 'u' => pig_latin_vowel(word),
        _ => pig_latin_non_vowel(word),
    }
}

fn pig_latin_vowel(word: String) -> String {
    let mut latin = String::from(word);
    latin.push_str("-");
    latin.push_str("hay");
    return latin;
}

fn pig_latin_non_vowel(word: String) -> String {
    let first_letter = word.chars().nth(0).unwrap();
    let mut latin = String::from(&word[1..]);
    latin.push_str("-");
    latin.push(first_letter);
    latin.push_str("ay");
    return latin;
}

#[test]
fn pig_latin_single_word() {
    let word = String::from("word");
    let latin = pig_latin(word);
    assert_eq!("ord-way", latin);
}

#[test]
fn pig_latin_different_single_word() {
    let word = String::from("hello");
    let latin = pig_latin(word);
    assert_eq!("ello-hay", latin);
}

#[test]
fn word_with_vowel_at_the_beginning() {
    let word = String::from("apple");
    let latin = pig_latin(word);
    assert_eq!("apple-hay", latin);
}
