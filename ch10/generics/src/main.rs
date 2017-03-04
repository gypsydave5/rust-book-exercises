fn main() {
    let point_float : Point<f32> = Point::new(1.1, 2.2);
    println!("point_float.x = {}, point_float.y = {}.", point_float.x(), point_float.y());

    let point_i32 : Point<i32> = Point::new(5, 6);
    println!("point_i32.x = {}, point_i32.y = {}.", point_i32.x(), point_i32.y());

    let point_u32 : Point<u32> = Point::new(7, 8);
    println!("point_u32.x = {}, point_u32.y = {}.", point_u32.x(), point_u32.y());
    println!("Sum of the float = {}.", point_float.x_plus_y());
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }

    pub fn new(x: T, y: T) -> Point<T> {
        Point { x: x, y: y }
    }
}

impl Point<f32> {
    pub fn x_plus_y(&self) -> f32 {
        self.x + self.y
    }
}

struct PointTwo<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointTwo<T, U> {
    pub fn mixup<V, W>(&self, other: PointTwo<V, W>) -> (PointTwo<T, W>, PointTwo<V, U>) {
        let a = PointTwo {
            x: self.x,
            y: other.y,
        };

        let b = PointTwo {
            x: other.x,
            y: self.y,
        };

        (a, b)
    }
}
