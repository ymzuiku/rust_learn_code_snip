use std::{sync::Arc, thread, time::Duration};

#[derive(Debug, PartialEq)]
struct Dog {
    name: String,
}

#[allow(dead_code)]
fn learn_vec() {
    let mut list = Vec::new();
    for _i in 1..5 {
        list.push(Dog {
            name: String::from(format!("{} {}", "aaaa", _i)),
        });
    }
    for v in list.iter() {
        println!("__debug__ {:?}", v);
    }
}

#[allow(dead_code)]
fn learn_spawn() {
    let v = Arc::new(vec![10000, 20000, 30000]);
    for _i in 1..50000 {
        let real_v = Arc::clone(&v);
        thread::spawn(move || {
            if real_v.contains(&_i) {
                println!("__debug__ in spawn {:?}", real_v.get(_i));
            } else {
                println!("__debug__ in spawn {:?}", _i);
            }
            // println!("__debug__ in spawn {:?}",);
            // thread::sleep(Duration::from_millis(5));
        });
    }

    thread::sleep(Duration::from_secs(1));

    for i in 1..5 {
        println!("__debug__ out {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    println!("__debug__ {}", "wait");
    // handle.join().unwrap();

    println!("__debug__ {}", "end");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_spawn() {
        // learn_vec();
        learn_spawn();
        assert_eq!(1, 1);
    }
}
