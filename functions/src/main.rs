fn main() {
    println!("Hello, world!");

    another_function();
    print(5);
    print_two(7, 8);
    curly_braces_are_expressions();

    let x = functions_are_expressions(5);
    println!("x is: {}", x);

    let x = ahh_good_old_increment(5);
    println!("x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}

fn print(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_two(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn curly_braces_are_expressions() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1 // expressions don't have semicolons at the end; statements do.
    };

    println!("The value of y is: {}", y);
}

fn functions_are_expressions(x: i32) -> i32 {
    x * 2 // don't stick a semicolon at the end of this; it won't get back to you
}

fn ahh_good_old_increment(x: i32) -> i32 {
    x + 1;
}
