fn main() {
    let nums = vec![5, 3, 7, 1, 10, 4];
    let result = largest(&nums);
    println!("The largest i32 is {}.", result);

    let chars = vec!['e', 'a', 's', 'y'];
    let result = largest(&chars);
    println!("The largest char is {}.", result);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &i in list.iter() {
        if i > largest {
            largest = i;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &i in list.iter() {
        if i > largest {
            largest = i;
        }
    }

    largest
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &i in list.iter() {
        if i > largest {
            largest = i;
        }
    }

    largest
}

