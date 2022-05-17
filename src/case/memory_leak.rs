use std::{cell::RefCell, rc::Rc};

type NextList = RefCell<Rc<List>>;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
enum List {
    Cons(i32, NextList),
    Nil,
}

use self::List::{Cons, Nil};

#[allow(dead_code)]
impl List {
    fn tail(&self) -> Option<&NextList> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
        println!("__debug__ a-1 {}", Rc::strong_count(&a));
        println!("{:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        println!("__debug__ a-2 {}", Rc::strong_count(&a));
        println!("__debug__ b-1 {}", Rc::strong_count(&b));
        println!("{:?}", b.tail());
        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("__debug__ b-2 {}", Rc::strong_count(&b));
        println!("__debug__ a-3 {}", Rc::strong_count(&a));

        // println!("{:?}", a.tail());

        assert_eq!(1, 1);
    }
}
