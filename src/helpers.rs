extern crate select;
extern crate url;

use select::document::Document;
use select::predicate::Name;
use url::{Position, Url};

pub fn get_base_url(url: &Url, doc: &Document) -> Url {
    let base_tag_href = doc
        .find(Name("base"))
        .filter_map(|node| node.attr("href"))
        .nth(0);

    let base_url = base_tag_href
        .map_or_else(|| Url::parse(&url[..Position::BeforePath]), Url::parse)
        .expect("");

    base_url
}
