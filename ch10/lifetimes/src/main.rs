use std::fmt::Display;

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    struct_with_lifetimes();

    longest_with_an_announcement(string1.as_str(), string2, "Woo-hoo!");
}


fn lifetime_fail() {
    // let r; // -- 'a lifetime starts

    // {
    //     let x = 5; // -- 'b lifetime starts
    //     r = &x;
    // } // -- 'b lifetime ends

    // println!("r: {}", r); // r refers to data from a lifetime that's ended -- compile error
}

// this fails because the borrow checked doesn't know whether the lifetime of the reference
// returned refers to `x` or `y` . It cannot refer to anything created in the scope - that stuff
// needs to either die or move out (and `str` not `&str` if it moves out).
// fn longest_fail(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn using_longest() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// This won't compile because you can't get a valid `result` when its lifetime, now limited by the
// lifetime of the `string2` argument to `longest`, cannot be guaranteed.
fn wont_work() {
    // let string1 = String::from("abcd");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }

    // println!("The longest string is {}", result);
}

// works because we're telling the borrow checker (checking the function) that every argument it's
// given will have at least as long lifetimes as each other - the `'a` lifetime bound limits
// downwards - it means that it's a least as long lived a. We are saying that the inputs and
// outputs should adhere to this contract, or be rejected by the borrow checker. We don't know how
// long any of the variables live, but we are saying that there is some valid lifetime (scope)
// called 'a which can hold.
//
// When 'real' values get passed in, 'a becomes the 'overlap' of the scopes - and if they don't
// overlap then boom. And scopes are always nested, so the intersection of the lifetimes of x and
// y will always be equivalent to either x or y.
//
// Which means that the lifetime of the returned `str` is only as long as the shortest lifetime of
// x or y.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Structs can have lifetime annotations for borrowed fields/references
// The lifetime of the struct is so limited to the lifetime of the reference it contains.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn struct_with_lifetimes() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a full stop");
    let i = ImportantExcerpt { part: first_sentence };
}


fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        return x
    }

    y
}
