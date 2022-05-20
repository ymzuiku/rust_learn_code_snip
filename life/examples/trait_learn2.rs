use std::str::FromStr;

use regex::Regex;

pub trait Parse {
    type E;
    fn parse(s: &str) -> Result<Self, Self::E>
    where
        Self: Sized;
}

impl<T> Parse for T
where
    T: FromStr + Default,
{
    type E = ();
    fn parse(s: &str) -> Result<Self, Self::E> {
        let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        if let Some(cap) = re.captures(s) {
            cap.get(0)
                .map_or(Err(()), |s| s.as_str().parse().map_err(|_err| ()))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_sould_work() {
        assert_eq!(u8::parse("123abcd"), Ok(123));
        assert_eq!(u8::parse("1abcd"), Ok(1));
        assert_eq!(f64::parse("125941.32aaa"), Ok(125941.32));
        assert_eq!(u32::parse("125941aaa"), Ok(125941));
        assert_eq!(u32::parse("0aaa"), Ok(0));
        assert_eq!(u32::parse("aaa"), Err(()));
    }
}

fn main() {
    match u8::parse("255 hello world") {
        Ok(v) => println!("result: {}", v),
        Err(e) => println!("no found:{:?}", e),
    }
}
