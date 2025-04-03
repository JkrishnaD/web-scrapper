# Web Scraper

A simple web scraper built in Rust that fetches and extracts text from web pages based on user-defined HTML elements. The extracted data is saved to a file for later use.

## Features

- **Fetch Web Data**: Scrape web pages by providing a URL.
- **Custom HTML Element Selection**: Extract text from user-specified HTML elements (e.g., `<p>`, `<h1>`, `<div>`).
- **Default Extraction Mode**: If no element is provided, extracts all readable text.
- **Save Extracted Data**: Stores scraped content into a text file.
- **CLI Argument Support**: Users can specify parameters via command-line arguments.
- **Error Handling**: Graceful error handling for invalid URLs and missing parameters.

## Installation

2. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/web_scraper.git
   cd web_scraper
   ```
3. Build the project:
   ```sh
   cargo build --release
   ```

## Usage

### Fetch Data and Extract Text

```sh
cargo run -- <URL> <HTML_ELEMENT> <FILE_NAME>
```

Example:

```sh
cargo run -- https://example.com p output.txt
```

### Fetch Data Without Specifying an Element (Extract All Text)

```sh
cargo run -- https://example.com
```

### Save Extracted Data to a Custom File

```sh
cargo run -- https://example.com p output.txt
```

## Contributing

Feel free to fork this repository and submit pull requests for new features or improvements.

## License

This project is licensed under the MIT License.

