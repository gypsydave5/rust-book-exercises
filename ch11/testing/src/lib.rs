#[test]
#[should_panic(expected = "do not lie on character bound")] // partial match of error message
fn slice_not_on_char_boundaries() {
    let s = "Здравствуйте";
    &s[0..1];
}

mod adding {
    // `cargo test adding` only runs the tests in adding
    #[test]
    //`cargo test add` will run anything with `add` in the function name
    fn add_two_and_two() {
        assert_eq!(2 + 2, 4)
    }

    #[test]
    fn add_one_and_three() {
        assert_eq!(1 + 3, 4)
    }
}

mod subtracting {
    #[test]
    fn subtract_three_from_four() {
        assert_eq!(4 - 3, 1)
    }
}

#[test]
#[ignore] // can be run with `cargo test -- --ignored`
fn expensive_boring_test() {
    assert_eq!(10 * 10, 100)
}
