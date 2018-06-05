fn main() {
    println!("Hello, world!");

    another_function();

    yet_another_function(5, 6);

    println!("The value of five is: {}", five());
}

fn another_function() {
    println!("Another function.");
}

fn yet_another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}
