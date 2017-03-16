fn main() {
    let mut x = vec![String::from("One"), String::from("Two"), String::from("Three")];

    //with borrow
    for i in &x {
        let mut a = "Number ".to_string();
        a.push_str(i);
        println!("The string is {}", a);
    }

    println!("The vector is still {:?}", x);

    //with mutate
    for i in &mut x {
        i.push_str("-ish");
        println!("Now the string is {}", i);
    }

    println!("The vector is now {:?}", x);

    //with consume (taking ownership)
    let y = for i in x {
        println!("I am EATING your STRING: {}", i);
    };

    println!("And a loop evaluates to: {:?}", y);
}
