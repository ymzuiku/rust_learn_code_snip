use std::{collections::HashMap, thread, time::Duration};

// fn sim(v: u32) -> u32 {
//     println!("__debug__ {:?}", "slowy");
//     thread::sleep(Duration::from_millis(1000));
//     v
// }

#[allow(dead_code)]
fn generate_workout(v: u32) {
    let mut n = 1;
    let mut sim = move |num| {
        n += 1;
        println!("__debug__ {:?}", format!("{} {}", "slowy", n));
        thread::sleep(Duration::from_millis(1));
        num
    };
    // let b = sim("333333");
    let v2 = println!("__debug__ <25 {:?}", sim(v));
    if v < 25 {
        println!("__debug__ <25 {:?}", v2);
        println!("__debug__ <25 next {:?}", v2);
    } else {
        println!("__debug__ >=25 {:?}", v2);
        println!("__debug__ >=25 next {:?}", v2);
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calc: T,
    value: HashMap<u32, bool>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    #[allow(dead_code)]
    fn new(calc: T) -> Cacher<T> {
        Cacher {
            calc: calc,
            value: HashMap::new(),
        }
    }
    #[allow(dead_code)]
    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            None => {
                let v = (self.calc)(arg);
                self.value.insert(v, true);
                v
            }
            Some(_) => arg,
        }
    }
}

#[allow(dead_code)]
fn generate_workout_use_struct(v: u32) {
    let n = 1;
    let mut sim = Cacher::new(move |num| {
        println!("__debug__ {:?}", num + n);
        thread::sleep(Duration::from_millis(1));
        num
    });
    let v2 = println!("__debug__ <25 {:?}", sim.value(v));
    if v < 25 {
        println!("__debug__ <25 {:?}", v2);
        println!("__debug__ <25 next {:?}", sim.value(v));
    } else {
        println!("__debug__ >=25 {:?}", v2);
        println!("__debug__ >=25 next {:?}", sim.value(v));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sim() {
        // let v = sim(3);
        let v = 3;
        println!("__debug__ {:?}", v);
        let v2 = generate_workout(v);
        println!("__debug__ {:?}", v2);

        let v3 = generate_workout_use_struct(5);
        println!("__debug__ {:?}", v3);
        // assert_eq!(1, 2);
    }
}
