fn main() {
    println!("f: {}", fibonacci(5));
}

fn fibonacci(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 2) + fibonacci(n - 1),
    }
}
