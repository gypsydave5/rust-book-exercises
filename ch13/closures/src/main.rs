fn main() {}

fn call_with_one<F>(some_closure: F) -> i32
    where F: Fn(i32) -> i32
{
    some_closure(1)
}

fn call_with_two<F>(some_closure: F) -> i32
    where F: Fn(&mut i32)
{
    let mut x = 2;
    some_closure(&mut x);
    x
}

fn call_it_twice<F>(mut f: F)
    where F: FnMut()
{
    f();
    f();
}

#[test]
fn functions_as_arguments() {
    let f = |x| x + 2;
    let answer = call_with_one(f);
    assert_eq!(3, answer);
}

#[test]
fn mutie_closures() {
    let f = |x: &mut i32| { *x = *x + 2; };
    let mut x = 5;
    f(&mut x);
    assert_eq!(x, 7);
}

#[test]
fn closure_scope() {
    let mut x = 5;

    {
        let mut f = |y| x = x + y;
        f(2);
    }

    assert_eq!(x, 7);
}

#[test]
fn mutie_closure_types() {
    let f = |x: &mut i32| { *x = *x + 2; };
    let answer = call_with_two(f);
    assert_eq!(4, answer);
}

#[test]
fn fnmut_type() {
    let mut x = 5;
    {
        let f = || x += 1;
        call_it_twice(f);
    }

    assert_eq!(x, 7);
}

#[test]
fn whaaaa() {
    let mut x = 5;
    x += 2;
    assert_eq!(x, 7);
}
