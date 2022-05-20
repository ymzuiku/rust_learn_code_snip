use std::fmt::format;

use sqlparser::dialect::Dialect;

#[derive(Debug, Clone, Default)]
pub struct TyrDialect;

impl Dialect for TyrDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
    }
    fn is_identifier_part(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch)
            || ('A'..='Z').contains(&ch)
            || ('0'..='9').contains(&ch)
            || [':', '/', '?', '&', '=', '-', '_', '.'].contains(&ch)
    }
}

pub fn example_sql() -> String {
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    format!(
        r#"select localtion name, total_case, new_case, total_deaths, new_deaths from {} where new_deaths >= 500 ORDER BY new_cases DESC LIMIT 6 OFFSET 5"#,
        url
    )
}

#[cfg(test)]
mod tests {
    use sqlparser::parser::Parser;

    use super::*;
    #[test]
    fn it_works() {
        let v = Parser::parse_sql(&TyrDialect::default(), &example_sql()).is_ok();
        assert!(v);

        let v2 = Parser::parse_sql(
            &TyrDialect::default(),
            "select localtion name, total > fromb",
        )
        .is_ok();
        assert!(v2);
    }
}
