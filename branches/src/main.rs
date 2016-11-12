fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    };

    some_craziness();
    expressions_in_if();
    different_types_in_arms();
}


fn some_craziness() {
    let number = 3;

    let x = if number < 5 {
        println!("condition was true"); // cute little sideeffect
        8
    } else {
        7
    };

    println!("here is x: {}", x);
}

fn expressions_in_if() {
    let x = if true == true { "bob" } else { "fred" };

    // No semicolon, block evaluates to the final expression
    // Two 'arms' to the if, so there can alwas be a value
    // So we can assign it
    //
    // And so if statements can be used like a ternary operator

    println!("here is x: {}", x);
}

fn different_types_in_arms() {
    let x = if true == true { "bob" } else { 5 };
    // if and else have incompatible types!
    // woot

    println!("here is x: {}", x);
}
