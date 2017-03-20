#[test]
fn simple_iterating() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, [2, 3, 4]);
}

pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn my_first_iterator_impl() {
    let mut c = Counter::new();
    let mut x = Vec::new();

    // needs to borrow mutably
    // because we're mutating c inside???
    for i in &mut c {
        x.push(i);
    }

    let a = c.next();

    assert_eq!(vec![1, 2, 3, 4, 5], x);
    assert_eq!(a, None);
}

#[test]
fn consuming_my_first_iterator() {
    // not mutable as gets consumed (death is not a change)
    let c = Counter::new();
    let a: Vec<u32> = c.take(5).collect();
    assert_eq!(a, vec![1, 2, 3, 4, 5]);
}

#[test]
fn playing_with_my_iterator() {
    let sum: u32 = Counter::new()
        .take(5)
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(18, sum);
}

struct Ints {
    n: u32,
}

impl Ints {
    fn new() -> Ints {
        Ints { n: 0 }
    }
}

impl Iterator for Ints {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.n += 1;
        return Some(self.n);
    }
}

fn factorial(n: usize) -> u32 {
    Ints::new().take(n).product()
}

#[test]
fn playing_with_iterators() {
    let a = factorial(5);
    assert_eq!(120, a);
}

struct Fibs {
    n: u32,
    p: u32,
}

impl Fibs {
    fn new() -> Fibs {
        Fibs { n: 0, p: 1 }
    }
}

impl Iterator for Fibs {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.n + self.p;
        self.p = self.n;
        self.n = x;
        return Some(self.n);
    }
}

#[test]
fn fib_test() {
    let f: u32 = Fibs::new().nth(4).expect("should be a number");
    assert_eq!(5, f);
}

fn fibby(n: usize) -> u32{
    match n {
        0 => 0,
        1 => 1,
        _ => fibby(n - 1) + fibby(n - 2),
    }
}

#[test]
fn more_fib_tests() {
    assert_eq!(8, fibby(6));
}

// struct Fab {
//     n: u32,
//     p: u32,
// }

// impl Iterator for Fab {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.zip(self.skip(1)).map(|(a, b)| a + b)
//     }

// }


