use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    even_shorter().unwrap();
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

fn unwrap_is_a_helper_on_results_that_panics_when_err() {
    let filename = "hello.txt";
    let f = File::open(filename).unwrap();
}

fn expect_is_like_unwrap_but_with_a_custom_error() {
    let filename = "hello.txt";
    let f = File::open(filename).expect(&format!("Failed to open {}", filename));
}

fn read_username_from_file() -> Result<String, io::Error> {
    // here we propagate the result (and the error);

    let filename = "hello.txt";
    let f = File::open(filename);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn read_username_from_file_with_question_mark() -> Result<String, io::Error> {
    // this is exactly the same as the above  - `?` acts like a match with an early return of
    // `Err<T>` values. Nice.
    let filename = "hello.txt";
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn even_shorter() -> Result<String, io::Error> {
    let filename = "hello.txt";
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    // revenge of the Elvis operator :D
    Ok(s)
}
