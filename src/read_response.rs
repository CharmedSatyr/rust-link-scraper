use reqwest::Response;
use select::document::Document;
use std::process;
use term::color::CYAN;
use url::Url;

pub fn read_response(response: Response) -> (Url, Document) {
    let mut t = term::stdout().unwrap();
    print!("\nReading ");
    t.fg(CYAN).unwrap();
    write!(t, "{}", response.url().as_str()).unwrap();
    t.reset().unwrap();
    println!("...");

    let url = Url::parse(response.url().as_str()).unwrap();
    let document = Document::from_read(response);
    let document = match document {
        Ok(doc) => doc,
        Err(e) => {
            println!("Error reading website: {}.", e);
            process::exit(1);
        }
    };
    (url, document)
}

#[cfg(test)]
mod tests {
    // TODO: Write tests. How to mock Response, Url, Document?
    // Simple test page: https://charmedsatyr.github.io/document/

    #[test]
    fn read_response_should_return_url_and_document_from_valid_response() {
        //
    }

    #[test]
    fn read_response_should_exit_with_message_on_invalid_response() {
        //
    }
}
