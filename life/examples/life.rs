pub fn strtok<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        return prefix;
    }

    let prefix = *s;
    *s = "";
    prefix
}

fn main() {
    let s = "hello world".to_string();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is: {}, s1:{}, s:{}", hello, s1, s);
}
