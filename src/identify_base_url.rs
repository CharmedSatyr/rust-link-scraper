use url::{Position::BeforePath, Url};

pub fn identify_base_url(url: &Url) -> Url {
    Url::parse(&url[..BeforePath]).expect("Unable to identify base URL")
}
