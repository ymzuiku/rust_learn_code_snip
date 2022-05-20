enum Language {
    Rust,
    Js,
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Language::Rust => "Rust",
            Language::Js => "Js",
        }
    }
}

fn print_ref(v: impl AsRef<str>) {
    println!("{}", v.as_ref())
}

fn main() {
    let lang = Language::Rust;
    print_ref("Hello world!");
    print_ref("Hello world!".to_string());
    print_ref(lang);
    print_ref(Language::Js);
    let b = Language::Rust.as_ref();
    println!("__debug__ {:?}", b);
}
