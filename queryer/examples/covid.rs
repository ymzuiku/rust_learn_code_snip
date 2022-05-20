use anyhow::Result;
use polars::prelude::*;
use std::{fs, io::Cursor};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    // let data = reqwest::get(url).await?.text().await?;
    // fs::write("covid.csv", data.as_bytes())?;
    // 节约测试时间，改为直接从本地读取，上面注释的代码为下载文件
    let data = include_bytes!("../covid.csv");

    let df = CsvReader::new(Cursor::new(data))
        .infer_schema(Some(16))
        .finish()?;
    let filtered = df.filter(&df["new_deaths"].gt(500))?;
    println!(
        "{:?}",
        filtered.select((
            "location",
            "total_cases",
            "new_cases",
            "total_deaths",
            "new_deaths"
        ))
    );

    Ok(())
}
