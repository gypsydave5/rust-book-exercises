use std::io::{self, Write};

fn main() {
    let s = grab_input("Farenheit or Celsius (f or c)").unwrap();
    let temperature: f32 = grab_input("Input temperature").unwrap().trim().parse().unwrap();

    let temperature = if s.trim() == "f" {
        convert_f_to_c(temperature)
    } else {
        convert_c_to_f(temperature)
    };

    println!("Temperature: {}", temperature);
}

fn grab_input(msg: &str) -> io::Result<String> {
    let mut buf = String::new();
    print!("{}: ", msg);

    try!(io::stdout().flush());
    try!(io::stdin().read_line(&mut buf));
    Ok(buf)
}

fn convert_f_to_c(f: f32) -> f32 {
    (f - 32.0) * (5.0 / 9.0)
}

fn convert_c_to_f(c: f32) -> f32 {
    (c / (5.0 / 9.0)) + 32.0
}
