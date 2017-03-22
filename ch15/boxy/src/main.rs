fn main() {
    boring_box();
}

fn boring_box() {
    let b = Box::new(5);
    println!("b = {}", b);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn listy() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

fn cons(x: i32, l: List) -> List {
    Cons(x, Box::new(l))
}

fn car(l: List) -> Option<i32> {
    match l {
        Cons(x, _) => Some(x),
        Nil => None
    }
}

fn cdr(l: List) -> List {
    match l {
        Cons(_, cdr) => *cdr,
        Nil => Nil,
    }
}
