fn main() {
    // Pennys
    let dime = Coin::Dime;
    let penny = Coin::Penny;
    println!("{}", value_in_cents(dime));
    println!("{}", value_in_cents_comedy(penny));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Arizona)));

    // Maths
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    placeholder();

    if_let();
    if_let_else();
}

fn using_some() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
    value_in_cents(Coin::Quarter(UsState::Arizona));
}

fn cannot_add_Option_to_not_option() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;
}

// Coins

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn value_in_cents_comedy(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}

// addition in Option

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn placeholder() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

// if let

fn if_let() {
    let some_option = Some(5);
    if let Some(x) = some_option {
        println!("{}", x);
    }
}

fn if_let_else() {
    let coin = Coin::Quarter(UsState::Alaska);

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        println!("Not a quarter");
    }
}
