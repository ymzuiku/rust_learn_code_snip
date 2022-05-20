use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
struct User<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    age: u8,
}

fn main() {
    let input = r#"{"name":"Tyr", "age":18}"#;
    let user: User = serde_json::from_str(input).unwrap();

    match user.name {
        Cow::Borrowed(v) => println!("borrowed {}", v),
        Cow::Owned(v) => println!("owned {}", v),
    }
}
