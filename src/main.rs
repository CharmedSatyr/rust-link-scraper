#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate select;

mod helpers;

use helpers::*;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

fn main() -> Result<()> {
    let response = handle_entry();

    joke(&response);

    let url_and_html = get_response(response);
    let url = url_and_html.0;
    let html = url_and_html.1;

    let base_url = get_base_url(&url, &html);

    let links_and_count = get_links(&html, &base_url);
    let links = links_and_count.0;
    let total_count = links_and_count.1;

    let unique_count = find_unique_links(&links);
    let broken_count = find_broken_links(&links);

    println!(
        "\nFound {} links, {} of which are unique and {} of which are broken.",
        total_count, unique_count, broken_count
    );

    Ok(())
}
