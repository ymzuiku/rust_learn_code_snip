#[allow(dead_code)]
fn add_one(x: i32) -> i32 {
    x + 1
}

#[allow(dead_code)]
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_do_twice() {
        let anser = do_twice(add_one, 5);
        println!("__debug__ {}", anser);
        assert_eq!(anser, 12);
    }
}
