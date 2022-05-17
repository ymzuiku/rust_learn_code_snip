pub trait Msg {
    fn send(&self, msg: &str);
}

#[derive(Debug, Clone)]
pub struct Limit<'a, T: Msg> {
    pub msg: &'a T,
    pub value: u32,
    pub max: u32,
}

impl<'a, T> Limit<'a, T>
where
    T: Msg,
{
    #[allow(dead_code)]
    pub fn new(msg: &T, max: u32) -> Limit<T> {
        Limit { msg, value: 0, max }
    }
    #[allow(dead_code)]
    pub fn set_value(&mut self, value: u32) {
        self.value = value;
        let per_max = self.value as f64 / self.max as f64;
        if per_max >= 1.0 {
            self.msg.send("error");
        } else if per_max >= 0.9 {
            self.msg.send("errorUrgent warning");
        } else if per_max > 0.75 {
            self.msg.send("warning");
        } else {
            self.msg.send("base");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    struct MockMsg {
        sent_msg: RefCell<Vec<String>>,
    }
    impl MockMsg {
        fn new() -> MockMsg {
            MockMsg {
                sent_msg: RefCell::new(vec![]),
            }
        }
    }
    impl Msg for MockMsg {
        fn send(&self, msg: &str) {
            // {
            //     let mut one = self.sent_msg.borrow_mut();
            //     one.push(String::from(msg));
            // }
            let mut two = self.sent_msg.borrow_mut();
            two.push(String::from(msg));
            // self.sent_msg.borrow_mut().push(String::from(msg));
            // self.sent_msg.borrow_mut().push(String::from(msg));
        }
    }
    #[test]
    fn test_msg() {
        let mock_msg = MockMsg::new();
        {
            let mut limit_tracker = Limit::new(&mock_msg, 100);
            limit_tracker.set_value(80);
            assert_eq!(mock_msg.sent_msg.borrow().len(), 1);
            limit_tracker.set_value(10);
            limit_tracker.set_value(10);
            let b = mock_msg.sent_msg.borrow();
            assert_eq!(b.len(), 3);
        }
        {
            let mut limit_tracker = Limit::new(&mock_msg, 100);
            limit_tracker.set_value(80);
            limit_tracker.set_value(20);
            // println!("__debug__ {:?}", limit_tracker);
            let a = mock_msg.sent_msg.borrow();
            let b = mock_msg.sent_msg.borrow();
            assert_eq!(a.len(), 5);
            assert_eq!(b.len(), 5);
        }
    }
}
