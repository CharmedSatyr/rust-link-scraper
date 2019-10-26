extern crate select;
extern crate url;

use reqwest::StatusCode;
use select::document::Document;
use select::predicate::Name;
use std::{collections::HashSet, io, process, thread, time};
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

pub fn print_unique_links(links: &HashSet<String>) -> () {
    println!("\nThe following unique URLs were found:");
    for link in links {
        println!("{}", link);
    }
}

pub fn find_broken_links(links: &HashSet<String>) -> usize {
    let mut broken_links = HashSet::new();

    for link in links {
        let res = reqwest::get(link).expect("\nError reading link.");

        match res.status() {
            StatusCode::NOT_FOUND => {
                broken_links.insert(link);
            }
            _ => (),
        };
    }

    if broken_links.len() > 0 {
        println!("\nAmong the unique URLs, the following pages may be broken or cause redirects:");
        for broken in &broken_links {
            println!("{}", broken);
        }
    }

    broken_links.len()
}

pub fn handle_entry() -> reqwest::Response {
    loop {
        println!("Enter a full url to scrape for links, e.g., \"https://www.nytimes.com\", or enter \"q\" to quit.");

        let mut entry = String::new();

        io::stdin()
            .read_line(&mut entry)
            .expect("Failed to read input!");

        let trimmed_entry = entry.trim();

        match trimmed_entry {
            "q" => process::exit(0),
            _ => (),
        };

        println!("Reading input...");

        let result = reqwest::get(trimmed_entry);

        match result {
            Ok(r) => {
                break r;
            }
            Err(e) => {
                println!("Invalid entry! Error: {}.", e);
                continue;
            }
        };
    }
}

pub fn joke(response: &reqwest::Response) {
    match response.url().as_str() {
        "https://www.streamate.com/"
        | "https://www.jerkmatelive.com/"
        | "https://www.youporn.com/"
        | "https://www.pornhub.com/" => {
            println!("Porn? You hound!");
            let sec = time::Duration::from_millis(1000);
            thread::sleep(sec);
        }
        _ => (),
    };
}
