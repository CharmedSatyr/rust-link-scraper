pub fn joke(url: &str) -> &str {
    match url {
        "https://streamate.com/"
        | "https://www.streamate.com/"
        | "https://www.jerkmatelive.com/"
        | "https://www.heidismembersonlyclub.com/"
        | "https://www.youpornlive.com/"
        | "https://www.pornhublive.com/"
        | "https://www.pornhub.com/"
        | "https://www.youporn.com/" => "\nPorn? You hound!",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::joke;

    #[test]
    fn joke_should_return_joke_for_porn_sites() {
        assert_eq!("\nPorn? You hound!", joke(&"https://www.pornhub.com/"));
    }

    #[test]
    fn joke_should_return_empty_string_for_other_sites() {
        assert_eq!("", joke(&"https://www.starbucks.com/"));
    }
}
