fn main() {
    let nums = vec![5, 3, 7, 1, 10, 4];
    let result = largest_i32(&nums);
    let result_generic = largest(&nums);
    println!("The largest i32 is {}.", result);
    println!("The largest i32 by generic is {}.", result_generic);

    let chars = vec!['e', 'a', 's', 'y'];
    let result = largest_char(&chars);
    let result_generic = largest(&chars);
    println!("The largest char is {}.", result);
    println!("The largest char by generic is {}.", result_generic);
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

fn largest<T:PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &i in list.iter() {
        if i > largest {
            largest = i;
        }
    }

    largest
}

