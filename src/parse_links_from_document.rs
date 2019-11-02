use select::{document::Document, predicate::Name};
use std::{collections::HashSet, process};
use url::{ParseError, Url};

pub fn parse_links_from_document(document: &Document, base_url: &Url) -> (HashSet<String>) {
    let mut links = HashSet::new();

    document
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|link| {
            match Url::parse(link) {
                Ok(url) => {
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
        println!("No links were found on that page.");
        process::exit(0);
    } else {
        println!("{} links found...", links.len());
    }

    links
}
