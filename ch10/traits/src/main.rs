fn main() {
    let nums = vec![5, 3, 7, 1, 10, 4];
    let result = largest_i32(&nums);
    let result_generic = largest(&nums);
    let result_clone = largest_clone(&nums);
    let result_reference = largest_reference(&nums);

    println!("The largest i32 is {}.", result);
    println!("The largest i32 by generic is {}.", result_generic);
    println!("The largest i32 by generic with clone is {}.", result_clone);
    println!("The largest i32 by generic by reference is {}.", result_reference);

    let chars = vec!['e', 'a', 's', 'y'];
    let result = largest_char(&chars);
    let result_generic = largest(&chars);
    let result_clone = largest_clone(&chars);
    let result_reference = largest_reference(&chars);

    println!("The largest char is {}.", result);
    println!("The largest char by generic is {}.", result_generic);
    println!("The largest char by generic with clone is {}.", result_clone);
    println!("The largest char by generic by reference is {}.", result_reference);
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

// returning a copy of the value in the array
fn largest<T:PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &i in list.iter() {
        if i > largest {
            largest = i;
        }
    }

    largest
}

// returning a clone of the value in the array
fn largest_clone<T:PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for i in list.iter() {
        if i > &largest {
            largest = i.clone()
        }
    }

    largest
}

// returning a reference to the value in the array
fn largest_reference<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for i in list.iter() {
        if i > largest {
            largest = i;
        }
    }

    largest
}

