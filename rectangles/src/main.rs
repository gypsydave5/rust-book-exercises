#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, r: &Rectangle) -> bool {
        self.length > r.length && self.width > r.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            length: size,
            width: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };
    let rect2 = Rectangle {
        length: 40,
        width: 10,
    };
    let rect3 = Rectangle {
        length: 45,
        width: 50,
    };

    println!("rect1 is {:?}", rect1);

    println!("The area of the rectangle is {} square pixels.",
             area(&rect1));

    println!("No, like, the area of the rectangle is {}px².",
             rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(5);

    println!("Here's a square: {:?}", sq);
}

fn area_params(length: u32, width: u32) -> u32 {
    length * width
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
