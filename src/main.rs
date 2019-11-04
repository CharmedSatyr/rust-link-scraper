#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate select;
extern crate term;

mod add_protocol;
mod get_link_status;
mod handle_entry;
mod identify_base_url;
mod joke;
mod parse_links_from_document;
mod read_response;

use get_link_status::*;
use handle_entry::*;
use identify_base_url::*;
use joke::*;
use parse_links_from_document::*;
use read_response::*;
use std::{thread, time};
use term::Attr::Bold;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

fn main() -> Result<()> {
    let mut t = term::stdout().unwrap();

    let response = handle_entry();

    let light_humor = joke(response.url().as_str());

    if light_humor.len() > 0 {
        println!("{}", light_humor);
        let pause = time::Duration::new(3, 0);
        thread::sleep(pause);
    }

    let url_and_document = read_response(response);
    let url = url_and_document.0;
    let document = url_and_document.1;

    let base_url = identify_base_url(&url);

    let links = parse_links_from_document(&document, &base_url);

    let counts = get_link_status(links);

    t.attr(Bold).unwrap();
    write!(
        t,
        "\nScraped {} total elements, {} of which are unique.",
        counts.0,
        counts.1 + counts.2 + counts.3
    )
    .unwrap();
    t.reset().unwrap();

    println!(
        "
‣ {} valid URLs
‣ {} redirects or not found responses
‣ {} invalid status codes or other elements\n",
        counts.1, counts.2, counts.3
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    // TODO: Write tests. How to spy on side effects?
    // Simple test page: https://charmedsatyr.github.io/document/

    #[test]
    fn main_has_desired_side_effects() {
        //
    }
}
