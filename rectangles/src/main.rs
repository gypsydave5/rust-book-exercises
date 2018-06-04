#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 30,
    };

    println!(
        "The area of the rectangle in {} square pixeles.",
        area(&rect)
    );

    println!("rect1 is {:?}", rect);
    println!("rect1 is {:#?}", rect);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
