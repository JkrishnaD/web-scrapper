use std::{ env, process };

use web_scrapper::{ extract_text, fetch, save_to_file, Config };

#[tokio::main]
async fn main() {
    let config = Config::url_parser(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing argument : {err}");
        process::exit(1);
    });

    println!("{}", config.url);
    let data = match fetch(&config.url).await {
        Ok(result) => extract_text(&result, config.element),
        Err(e) => {
            return println!("error while fetching the data {e}");
        }
    };

    save_to_file(&data, &config.filename);
}
