use std::ops::{Deref, DerefMut};

use anyhow::Result;
use polars::{
    io::SerWriter,
    prelude::{CsvWriter, DataFrame},
};

pub mod convert;
pub mod dialect;
pub mod fetcher;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[derive(Debug)]
pub struct DataSet(DataFrame);

impl Deref for DataSet {
    type Target = DataFrame;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DataSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DataSet {
    pub fn to_csv(&self) -> Result<String> {
        let mut buf = Vec::new();
        let writer = CsvWriter::new(&mut buf);
        writer.finish(self)?;
        Ok(String::from_utf8(buf)?)
    }
}
