use url::{Position::BeforePath, Url};

pub fn identify_base_url(url: &Url) -> Url {
    Url::parse(&url[..BeforePath]).expect("Unable to identify base URL")
}

#[cfg(test)]
mod tests {
    use super::{identify_base_url, Url};

    #[test]
    fn identify_base_url_should_return_base_url_from_base_url() {
        let url = Url::parse("https://www.example.com/").expect("");

        assert_eq!(url, identify_base_url(&url));
    }

    #[test]
    fn identify_base_url_should_return_base_url_from_longer_url() {
        let base_url = Url::parse("https://www.example.com/").expect("");
        let long_url = Url::parse("https://www.example.com/path/to/somewhere/").expect("");

        assert_eq!(base_url, identify_base_url(&long_url));
    }
}
