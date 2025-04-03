use std::{ fs::File, io::Write };

use reqwest;
use scraper::{ Html, Selector };

pub struct Config {
    pub url: String,
    pub element: String,
    pub filename: String,
}

// method to get the arguments from the run command
impl Config {
    pub fn url_parser(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let url = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("cannot parse the command");
            }
        };

        let element = args.next().unwrap_or_else(|| "body".to_string());

        let filename = args.next().unwrap();
        Ok(Config { url, element, filename })
    }
}

pub async fn fetch(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

pub fn extract_text(html: &str, element: String) -> String {
    let document = Html::parse_document(html);
    let selector = Selector::parse(&element).unwrap();

    let mut extracted_text = String::new();
    println!("hello from the extract text");
    for element in document.select(&selector) {
        extracted_text.push_str(&element.text().collect::<Vec<_>>().join(" "));
        extracted_text.push('\n');
    }

    extracted_text
}

pub fn save_to_file(data: &str, filename: &str) {
    let mut file = File::create(filename).expect("Failed to create file");

    file.write_all(data.as_bytes()).expect("Failed to add the data")
}
