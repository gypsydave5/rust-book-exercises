#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn cons(x: i32, l: List) -> List {
    Cons(x, Box::new(l))
}

pub fn car(l: List) -> Option<i32> {
    match l {
        Cons(x, _) => Some(x),
        Nil => None,
    }
}

pub fn cdr(l: List) -> List {
    match l {
        Cons(_, cdr) => *cdr,
        Nil => Nil,
    }
}

impl List {
    pub fn new(l: &[i32]) -> List {
        l.iter().rev().fold(List::Nil, |acc, &x| cons(x, acc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cons() {
        assert_eq!(Cons(5, Box::new(Nil)), cons(5, Nil));
    }

    #[test]
    fn test_multi_cons() {
        let result = cons(1, cons(2, cons(3, Nil)));
        let expected = list_fixture();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_car() {
        let list = list_fixture();
        assert_eq!(car(list), Some(1));
    }

    #[test]
    fn test_cdr() {
        let list = list_fixture();
        let expected = Cons(2, Box::new(Cons(3, Box::new(Nil))));
        assert_eq!(expected, cdr(list))
    }

    #[test]
    fn test_new_list() {
        let list = list_fixture();
        let new_list = List::new(&[1, 2, 3]);
        assert_eq!(list, new_list);
    }

    fn list_fixture() -> List {
        Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))
    }
}
