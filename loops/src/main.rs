fn main() {
    while_no_while();
    some_while();
    countdown();
}

fn to_infinity() {
    loop {
        println!("and beyond!");
    }
}

fn while_no_while() {
    let mut n = 0;

    loop {
        if n == 3 {
            break;
        }
        n = n + 1;
        println!("Looping for the {} time", n);
    }
}

fn some_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

fn whiling_away_the_index() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }
}

fn for_the_win() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn countdown() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
