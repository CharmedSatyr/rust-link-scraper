#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate select;
extern crate term;

mod add_protocol;
mod check_link_status;
mod get_base_url;
mod get_links;
mod get_response;
mod handle_entry;
mod joke;

use check_link_status::*;
use get_base_url::*;
use get_links::*;
use get_response::*;
use handle_entry::*;
use joke::*;
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

    let url_and_html = get_response(response);
    let url = url_and_html.0;
    let html = url_and_html.1;

    let base_url = get_base_url(&url, &html);

    let links = get_links(&html, &base_url);

    let counts = check_link_status(links);

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
