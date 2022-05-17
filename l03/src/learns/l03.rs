// https://time.geekbang.org/column/article/411632
// 03｜初窥门径：从你的第一个Rust程序开始！

use std::error::Error;
use std::fs;

#[allow(dead_code)]
fn l03() -> Result<(), Box<dyn Error>> {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";
    println!("fetching url:{}", url);

    let body = reqwest::blocking::get(url)?.text()?;
    println!("html to markdown...");
    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes())?;
    println!("converted markdown has been saved in {}", output);
    Ok(())
}

#[allow(dead_code)]
fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

#[allow(dead_code)]
fn square(value: i32) -> i32 {
    value * value
}

#[allow(dead_code)]
fn cube(value: i32) -> i32 {
    value * value * value
}

#[allow(dead_code)]
fn pi() -> f64 {
    3.14
}

#[allow(dead_code)]
fn not_pi() {
    3.14;
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Gender {
    Unspecified = 0,
    Female,
    Male,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct UserId(u64);

#[derive(Debug, Clone, Copy, PartialEq)]
struct TopicId(u64);

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

#[allow(dead_code)]
fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);
    // 最常用的 for 循环
    for _i in 2..n {
        let c = a + b;
        if _i > 3 {
            break;
        }
        a = b;
        b = c;
        println!("next val is {}", b);
    }
}

#[allow(dead_code)]
fn process_event(event: &Event) {
    match event {
        Event::Join((uid, _tid)) => println!("user {:?} joined", uid),
        Event::Leave((uid, _tid)) => println!("user {:?} leaved", uid),
        Event::Message((_uid, _tid, msg)) => println!("user {:?} msg", msg),
    }
}

fn process_event_let(event: &Event) {
    if let Event::Message((_, _, msg)) = event {
        println!("broadcast: {}", msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_l03() {
        // l03()
        assert_eq!(4, apply(2, square));
        assert_eq!(8, apply(2, cube));

        let the_pi = pi();
        assert_eq!(3.14, the_pi);

        let the_notpi = not_pi();
        assert_eq!((), the_notpi);
    }

    #[test]
    fn test_emun() {
        let alice = User {
            id: UserId(1),
            name: "alice".into(),
            gender: Gender::Female,
        };
        let bob = User {
            id: UserId(2),
            name: "Bob".into(),
            gender: Gender::Male,
        };
        let topic = Topic {
            id: TopicId(1),
            name: "rust".into(),
            owner: UserId(1),
        };
        let event1 = Event::Join((alice.id, topic.id));
        let event2 = Event::Join((bob.id, topic.id));
        let event3 = Event::Message((alice.id, topic.id, "hello world!".into()));

        process_event(&event2);
        process_event_let(&event3);
        println!("__debug__ {:?} {:?}", event2, event3);
        assert_eq!(event1, Event::Join((UserId(1), topic.id)));
    }

    #[test]
    fn test_fib() {
        fib_for(5);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_range() {
        let arr = [1, 2, 3, 4];
        assert_eq!(arr[..], [1, 2, 3, 4]);
        assert_eq!(arr[0..3], [1, 2, 3]);
    }
}
