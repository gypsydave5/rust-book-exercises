mod subtractor;

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

pub fn subtract_two(minuend: i32) -> i32 {
    subtractor::subtract(minuend, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)] // only compiles for the test runner - not on build
mod tests {
    use add_two; // need to bring the tested code into scope
    use subtract_two;
    use internal_adder;

    mod addition {
        #[test]
        fn can_add_two() {
            assert_eq!(4, add_two(2))
        }
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2))
    }

    mod subtraction {
        #[test]
        fn can_subtract_two() {
            assert_eq!(2, subtract_two(4))
        }
    }
}
