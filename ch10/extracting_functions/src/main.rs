// Dealing with duplication using function extraction

fn main() {
    two_lists_duplicated_code();
    two_lists_refactored();
}

// BAD! BAD! Naughty duplicated code! NO!
fn two_lists_duplicated_code() {
    let numbers = vec![34, 50, 25, 100, 65];

    let mut largest = numbers[0];

    for number in numbers {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let numbers = vec![102, 34, 6000, 89, 54, 1, 43, 8];

    let mut largest = numbers[0];

    for number in numbers {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];


    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn two_lists_refactored() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let numbers = vec![102, 34, 6000, 89, 54, 1, 43, 8];
    let result = largest(&numbers);
    println!("The largest number is {}", result);
}
