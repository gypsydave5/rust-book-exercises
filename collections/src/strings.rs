pub fn new_string_from_data() {
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
}

pub fn utf8_strings() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    "안녕하세요".to_string();
}

pub fn updating_strings() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    s.push('b');
    s.push('a');
    s.push('z');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}

pub fn length() {
    let hello = "hello";
    println!("{} len = {}", hello, hello.len());
    let hello = "Здравствуйте";
    println!("{} len = {}", hello, hello.len());
}

pub fn iterating() {
    let s = String::from("नमस्ते");

    //over characters
    for c in s.chars() {
        println!("{}", c);
    }

    //over bytes
    for b in s.bytes() {
        println!("{}", b);
    }
}
