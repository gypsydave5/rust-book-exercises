fn main() {
    let width1 = 30;
    let width2 = 50;

    println!(
        "The area of the rectangle in {} square pixeles.",
        area(width1, width2)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
