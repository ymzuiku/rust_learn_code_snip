use std::fmt::Display;

#[derive(Debug, Clone, Default)]
struct Dog {
    name: String,
    age: u8,
}

impl Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "aaa: {} {}", self.name, self.age)
    }
}

fn main() {
    let d = Dog {
        name: "fff".into(),
        ..Default::default()
    };
    println!("__debug__ {}", d.to_string());
    println!("__debug__ {}", d);
}
