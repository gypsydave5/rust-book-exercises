#[test]
fn create_empty_string() {
    let s = String::new();
    assert_eq!("", s);
}

#[test]
fn initialize_string_with_to_string() {
    let data = "string";
    let s = data.to_string();
    assert_eq!("string", s);

    let s = "one line".to_string();
    assert_eq!("one line", s);
}

#[test]
fn initialize_string_with_from() {
    let s = String::from("string");
    assert_eq!("string", s);
}

#[test]
fn strings_are_all_utf() {
    let s = String::from("안녕하세요");
    assert_eq!("안녕하세요", s);
}

#[test]
fn push_str_to_append() {
    // need to be able to _mut_ ate ;)
    let mut s = String::from("foo");
    let s2 = "bar".to_string();
    // push_str takes a borrowed (&) string slice (str)
    s.push_str(&s2);
    assert_eq!("foobar", s);
}

#[test]
fn push_to_append_char() {
    let mut s = String::from("rus");
    s.push('t');
    assert_eq!("rust", s);
}

#[test]
fn concat_with_plus() {
    let s1 = String::from("foo");
    let s2 = String::from("bar");
    // needs to be borrowed
    let s3 = s1 + &s2;
    assert_eq!("foobar", s3);
    // s2 can still be used; it was only borrowed
    // s1 cannot; it has now been moved
}

// + is using `add` under the hood
#[test]
fn concat_with_add() {
    use std::ops::Add;
    // we look forward to finding out why we need to do this
    // but we are in love with the language that tells us we need to

    let s1 = String::from("foo");
    let s2 = String::from("bar");
    // needs to be borrowed
    let s3 = s1.add(&s2);

    assert_eq!("foobar", s3);
    // s2 can still be used; it was only borrowed
    // s1 cannot; it has now been moved to add
}

#[test]
fn format_macro() {
    let s1 = String::from("aye");
    let s2 = String::from("bee");
    let s3 = String::from("sea");

    let abc = format!("{}-{}-{}", s1, s2, s3);

    assert_eq!("aye-bee-sea", abc)
}

#[test]
fn no_index_string() {
    let s = "index".to_string();
    // this will not compile:
    // let x = s[4];
}

#[test]
fn string_len_in_bytes() {
    let s = "hello".to_string();
    assert_eq!(5, s.len());
    // one byte per character (like ASCII)

    let s = "Союз".to_string();
    assert_eq!(8, s.len());
    // not 4... 2 bytes per character
}

#[test]
fn can_index_by_range() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    assert_eq!("Зд", s);
}

#[test]
#[should_panic]
fn panics_when_range_not_on_character_boundary() {
    let hello = String::from("Здравствуйте");
    let s = &hello[0..5];
    assert_eq!("Зд", s);
}

#[test]
fn string_iterator_chars() {
    let mut chars: Vec<char> = Vec::new();
    let hi = String::from("hi");

    for c in hi.chars() {
        chars.push(c);
    }

    assert_eq!('h', chars[0]);
    assert_eq!('i', chars[1]);
}

#[test]
fn string_iterator_bytes() {
    let mut bytes: Vec<u8> = Vec::new();
    let hello = "Здравствуйте".to_string();

    for b in hello.bytes() {
        bytes.push(b);
    }

    assert_eq!(208, bytes[0]);
    assert_eq!(151, bytes[1]);
}
