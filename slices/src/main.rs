fn main() {
    let mut s = String::from("hello world");
    let word = better_first_word(&s);
    println!("First word: {}", word);

    let s = "hello world";
    let word = better_first_word(&s);
    println!("First word: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn better_first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
