fn main() {
    let string = String::from("hello world, this isn't Margaret");
    let string_literal = "a string literal";

    let s = first_word(&string);
    println!("{}", s);
    let s = second_word(&string);
    println!("{}", s);

    let s = first_word(&string_literal);
    println!("{}", s);
    let s = second_word(&string_literal);
    println!("{}", s);

}

fn first_word_no_slice(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn string_slices() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut count = 0;
    let mut last = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            count = count + 1;

            if count == 2 {
                return &s[last..i];
            } else {
                last = i + 1;
            }
        }
    }
    &s[..]
}
