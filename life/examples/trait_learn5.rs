#[derive(Debug, Clone, Copy)]
struct Developer<'a> {
    name: &'a str,
    age: u8,
    lang: Language,
}

#[derive(Debug, Clone, Copy)]
enum Language {
    Rust,
    JS,
}

fn main() {
    let dev = Developer {
        name: "try",
        age: 18,
        lang: Language::Rust,
    };
    let dev1 = dev;
    println!("dev: {:?} addr: {:p}", dev, dev.name);
    println!("dev1: {:?} addr: {:p}", dev1, dev1.name);
}
