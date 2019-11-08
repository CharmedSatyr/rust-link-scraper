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

    if !links.is_empty() {
        println!("No links were found on that page.");
        process::exit(0);
    } else {
        println!("{} links found...", links.len());
    }

    links
}

#[cfg(test)]
mod tests {
    // TODO: Write tests. How to mock Document?
    // Simple test page: https://charmedsatyr.github.io/document/

    #[test]
    fn parse_links_from_document_returns_hashset_of_strings_if_document_contains_links() {
        //
    }

    #[test]
    fn parse_links_from_document_exits_with_message_if_document_contains_no_links() {
        //
    }
}
