extern crate rand;

mod vectors;
mod strings;
mod hashmaps;
mod exercises;

fn main() {
    use rand::prelude::*;

    vectors::accessing_elements();
    vectors::iteration();
    vectors::mutable_iteration();

    strings::new_string_from_data();
    strings::utf8_strings();
    strings::updating_strings();
    strings::length();
    strings::iterating();

    hashmaps::blue_vs_yellow();
    hashmaps::upsert();

    let mut rng = rand::thread_rng();
    let numbers: Vec<u32> = (1..20).into_iter().map(|_| rng.gen_range(1, 10)).collect();
    println!("{:?}", numbers);

    let average = exercises::average(numbers);
    println!("Mode: {}", average.mode);
    println!("Median: {}", average.median);
}
