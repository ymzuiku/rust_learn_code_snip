// pub trait Formatter {
//     fn format(&self, input: &mut String) -> bool;
// }

// struct MarkdownFormatter;
// impl Formatter for MarkdownFormatter {
//     fn format(&self, input: &mut String) -> bool {
//         input.push_str("\nformatted with Markdown");
//         true
//     }
// }

// struct RustFormatter {}
// impl Formatter for RustFormatter {
//     fn format(&self, input: &mut String) -> bool {
//         input.push_str("\nformatted with Rust");
//         true
//     }
// }

// struct HtmlFormatter;
// impl Formatter for HtmlFormatter {
//     fn format(&self, input: &mut String) -> bool {
//         input.push_str("\nformatted with HTML");
//         true
//     }
// }

// pub fn format(input: &mut String, formatters: Vec<&dyn Formatter>) {
//     for formatter in formatters {
//         formatter.format(input);
//     }
// }

// fn main() {
//     let mut text = "hello world".to_string();
//     let html: &dyn Formatter = &HtmlFormatter {};
//     let rust: &dyn Formatter = &RustFormatter {};

//     let formatters = vec![html, rust];
//     format(&mut text, formatters);
//     println!("text: {}", text);
// }

use std::{fs::File, io::Write};

fn main() {
    let mut f = File::create("./test_trait").unwrap();
    let w: &mut dyn Write = &mut f;
    w.write_all(b"hello ").unwrap();
    let w1 = w.by_ref();
    w1.write_all(b"world").unwrap();
}
