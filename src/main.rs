#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate select;
extern crate term;

mod helpers;

use helpers::*;
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

    joke(&response);

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
