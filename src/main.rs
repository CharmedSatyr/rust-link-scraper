#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate select;

mod helpers;

use helpers::{find_broken_links, get_base_url, handle_entry, joke, print_unique_links};
use select::document::Document;
use select::predicate::Name;
use std::{collections::HashSet, process};
use url::{ParseError, Url};

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

fn main() -> Result<()> {
    let response = handle_entry();

    joke(&response);

    println!("Finding links in \"{}\"...", response.url().as_str());

    let url = Url::parse(response.url().as_str()).unwrap();
    let document = Document::from_read(response);
    let document = match document {
        Ok(doc) => doc,
        Err(e) => {
            println!("Error reading website: {}.", e);
            process::exit(1);
        }
    };

    let base_url = get_base_url(&url, &document);

    let mut links = HashSet::new();
    let mut count = 0;

    document
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|link| {
            match Url::parse(link) {
                Ok(url) => {
                    count += 1;
                    links.insert(url.as_str().to_owned());
                }
                Err(ParseError::RelativeUrlWithoutBase) => {
                    let l = base_url
                        .join(link)
                        .expect("Trouble resolving relative URL.");
                    links.insert(l.as_str().to_owned());
                }
                Err(e) => println!("Could not parse \"{}\".", e),
            };
        });

    if links.len() == 0 {
        println!("No links were found.");
        process::exit(0);
    }

    print_unique_links(&links);

    let broken_count = find_broken_links(&links);

    println!(
        "\nFound {} links, {} of which are unique, and {} of which are broken.",
        count,
        links.len(),
        broken_count
    );

    Ok(())
}
