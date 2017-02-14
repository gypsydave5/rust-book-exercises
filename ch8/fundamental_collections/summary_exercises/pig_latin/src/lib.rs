pub fn pig_latin(word: String) -> String {
    let first_letter = word.chars().nth(0).unwrap();

    if is_vowel(first_letter) {
        pig_latin_vowel(word)
    } else {
        pig_latin_non_vowel(word)
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

fn is_vowel(c: char) -> bool {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    return vowels.iter().any(|x| *x == c);
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

#[test]
fn is_vowel_for_non_vowel() {
    let non_vowel = 'c';
    assert!(!is_vowel(non_vowel));
}

#[test]
fn is_vowel_for_vowel() {
    let vowel = 'a';
    assert!(is_vowel(vowel));
}
