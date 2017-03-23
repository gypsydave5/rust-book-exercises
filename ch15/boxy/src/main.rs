extern crate boxy;

fn main() {
    boring_box();
}

fn boring_box() {
    let b = Box::new(5);
    println!("b = {}", b);
}

