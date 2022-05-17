use std::fs;

#[allow(dead_code)]
fn l03() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";
    println!("fetching url:{}", url);

    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("html to markdown...");
    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes()).unwrap();
    println!("converted markdown has been saved in {}", output)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_l03() {
        // l03()
        assert_eq!(4, apply(2, square));
        assert_eq!(8, apply(2, cube));
    }
}
