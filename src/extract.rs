use std::fs::File;
use std::io::copy;
use std::path::Path;

extern crate reqwest;

async fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    let path = Path::new(file_path);
    let mut file = File::create(&path)?;

    let content = response.bytes().await?;
    copy(&mut &content[..], &mut file)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://gist.githubusercontent.com/noamross/e5d3e859aa0c794be10b/raw/b999fb4425b54c63cab088c0ce2c0d6ce961a563/cars.csv";
    let file_path = "tables/cars.csv";
    extract(url, file_path).await?;

    Ok(())
}