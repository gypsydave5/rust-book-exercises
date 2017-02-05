fn main() {
    let s1 = String::from("hello");
    let (s1, len) = worlds_most_obtuse_len(s1);
    println!("The length of '{}' is {}.", s1, len);

    let len = a_much_better_len(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = s1;
    can_change_a_mutable_reference(&mut s2);
    println!("{}", s2);

    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// fn no_simultaneous_mutable_references() {
//     let mut s = String::from("mutate me");

//     let r1 = &mut s;
//     let r2 = &mut s;
// }

fn get_away_with_multiple_borrows_across_scopes() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    }

    let r2 = &mut s;
}

// fn cannot_borrow_mutable_and_immutable() {
//     let mut s = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;
//     let r3 = &mut s;
// }

// fn cannot_change_a_reference(some_string: &String) {
//     some_string.push_str(" - not going to happen");
// }

fn can_change_a_mutable_reference(some_string: &mut String) {
    some_string.push_str(" - yeah, we're good");
}

fn ownership_one() {
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn values_and_scope() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn worlds_most_obtuse_len(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn a_much_better_len(s: &String) -> usize {
    s.len()
}

fn fun_with_strings() -> String {
    let mut s = String::from("hello");
    s.push_str(", world");
    s.push('!');
    let s2 = s.clone();
    s2
}

fn return_tuple(x: i32, y: i32) -> (i32, i32) {
    (x, y)
}
