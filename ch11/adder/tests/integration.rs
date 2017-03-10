extern crate adder;
mod helpers; // inculding helper functions

#[test]
fn it_adds_two() {
    let x = helpers::the_number_two();
    assert_eq!(4, adder::add_two(x))
}
