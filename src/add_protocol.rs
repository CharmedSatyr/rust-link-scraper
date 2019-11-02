pub fn add_protocol(trimmed_entry: &str) -> String {
    let insecure = String::from("http://");
    let mut with_protocol = String::from("https://");

    let mut result = trimmed_entry;
    if (trimmed_entry.len() > 7
        && trimmed_entry[..8] != with_protocol
        && trimmed_entry[..7] != insecure)
        || (trimmed_entry.len() <= 7)
    {
        with_protocol.push_str(trimmed_entry);
        result = &with_protocol;
    }
    String::from(result)
}

#[cfg(test)]
mod tests {
    use super::add_protocol;

    #[test]
    fn add_protocol_adds_https_when_missing_protocol() {
        assert_eq!("https://example.com", add_protocol("example.com"));
    }

    #[test]
    fn add_protocol_ignores_sites_with_https_protocol() {
        assert_eq!("https://example.com", add_protocol("https://example.com"));
    }

    #[test]
    fn add_protocol_ignores_sites_with_http_protocol() {
        assert_eq!("http://example.com", add_protocol("http://example.com"));
    }
}
