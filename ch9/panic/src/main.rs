use std::fs::File;
use std::io::ErrorKind;

fn main() {
    matching_on_different_types_of_error();
}

fn this_kinda_thing_panics() {
    let v = vec![1, 2, 3];

    v[100];
}

fn this_kinda_thing_gives_a_result() {
    // fun way to find out a type - give it the wrong type and compile!
    // let file: u32 = File::open("hello.txt");

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("There was an error opening the file: {:?}", error),
    };
}

fn matching_on_different_types_of_error() {
    let filename = "hello.txt";
    let f = File::open(filename);
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create(filename) {
                Ok(fc) => fc,
                Err(error) => panic!("There was an error creating file: {:?}", error),
            }
        }
        Err(error) => panic!("There was an error opening the file: {:?}", error),
    };
}
