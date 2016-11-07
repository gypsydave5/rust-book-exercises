fn main() {
    shadowing();
    numeric_operations();
    booleans();
    characters();
    tuples();
    arrays();
    panic_at_the_out_of_bounds_disco();
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "      ";
    println!("The value of spaces is: {}", spaces);

    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}

fn numeric_operations() {
    let sum: u16 = 5 + 10;
    println!("sum: {}", sum);

    let difference: f32 = 95.5 - 4.3;
    println!("difference: {}", difference);

    let product = 4 * 30;
    println!("product: {}", product);

    let quotient = 56.7 / 32.2;
    println!("quotient: {}", quotient);

    let remainder = 43 % 5;
    println!("remainder: {}", remainder);

    let binary: u32 = 0b0100;
    println!("binary: {}", binary);
}

fn booleans() {
    let t = true;
    let f: bool = false;

    println!("(T & F) === {}", t && f);
}

fn characters() {
    let c = 'z';
    let z = 'Z';

    let heart_eyed_cat = '😻';

    println!("characters: {} {} {}", c, z, heart_eyed_cat);
}

fn tuples() {
    let tup: (i32, f64, u8, char) = (500, 6.4, 1, '\n');
    let (x, _, _, _) = tup;

    println!("The value of x is: {}", x);

    println!("The second value is: {}", tup.1);

    println!("The final value is: {}", tup.3);
}

fn arrays() {
    let a = [1, 2, 3, 4, 5];
    let e = a[2];

    println!("The second element is: {}", e);
}

fn panic_at_the_out_of_bounds_disco() {
    let a = [1, 2, 3, 4, 5];
    let e = a[99];
}
