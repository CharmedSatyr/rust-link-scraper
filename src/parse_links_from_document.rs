use select::{document::Document, predicate::Name};
use std::collections::HashSet;
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

    links
}

#[cfg(test)]
mod tests {
    use super::{parse_links_from_document, Document, HashSet, Url};

    #[test]
    fn parse_links_from_document_returns_hashset_of_strings_if_document_contains_links() {
        let document = Document::from(include_str!("./mocks/test_with_link.html"));
        let base_url = Url::parse("https://www.example.com/").unwrap();

        let mut mock_hash_map = HashSet::new();
        mock_hash_map.insert(String::from("https://www.example.com/"));

        let fn_result = parse_links_from_document(&document, &base_url);

        assert_eq!(mock_hash_map, fn_result);
    }

    #[test]
    fn parse_links_from_document_returns_empty_hashset_if_document_contains_no_links() {
        let document = Document::from(include_str!("./mocks/test_no_link.html"));
        let base_url = Url::parse("https://www.example.com/").unwrap();

        let fn_result = parse_links_from_document(&document, &base_url);

        assert!(fn_result.len() == 0);
    }
}
