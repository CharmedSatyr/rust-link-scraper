extern crate select;
extern crate url;

use reqwest::StatusCode;
use select::document::Document;
use select::predicate::Name;
use std::{collections::HashSet, io, process, thread, time};
use url::{ParseError, Position, Url};

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
        | "https://www.heidismembersonlyclub.com/"
        | "https://www.youpornlive.com/"
        | "https://www.pornhublive.com/" => {
            println!("\nPorn? You hound!\n");
            let pause = time::Duration::new(3, 0);
            thread::sleep(pause);
        }
        _ => (),
    };
}

pub fn get_response(response: reqwest::Response) -> (Url, Document) {
    println!("\nReading \"{}\"...", response.url().as_str());
    let url = Url::parse(response.url().as_str()).unwrap();
    let document = Document::from_read(response);
    let document = match document {
        Ok(doc) => doc,
        Err(e) => {
            println!("Error reading website: {}.", e);
            process::exit(1);
        }
    };
    (url, document)
}

pub fn get_links(document: &Document, base_url: &Url) -> (HashSet<String>, i32) {
    let mut links = HashSet::new();
    let mut total_count = 0;

    document
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|link| {
            match Url::parse(link) {
                Ok(url) => {
                    total_count += 1;
                    links.insert(url.as_str().to_owned());
                }
                Err(ParseError::RelativeUrlWithoutBase) => {
                    total_count += 1;
                    let l = base_url
                        .join(link)
                        .expect("Trouble resolving relative URL.");
                    links.insert(l.as_str().to_owned());
                }
                Err(e) => println!("Could not parse \"{}\".", e),
            };
        });

    if total_count == 0 {
        println!("No links were found on that page.");
        process::exit(0);
    }

    (links, total_count)
}

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

pub fn find_unique_links(links: &HashSet<String>) -> usize {
    println!("\nThe following unique URLs were found:");
    let mut count = 0;
    for link in links {
        count += 1;
        println!("{}", link);
    }
    count
}

pub fn find_broken_links(links: &HashSet<String>) -> usize {
    println!("\nLooking for broken links...");
    let mut broken_links = HashSet::new();

    for link in links {
        let res = reqwest::get(link); //.expect("\nError reading link.");

        match res {
            Ok(r) => {
                if r.status() == StatusCode::NOT_FOUND {
                    broken_links.insert(link);
                };
            }
            Err(_) => {
                // Likely an email address or script
                continue;
            }
        }
    }

    if broken_links.len() > 0 {
        println!("\nAmong the unique URLs, the following pages are broken or cause redirects:");
        for broken in &broken_links {
            println!("{}", broken);
        }
    }

    broken_links.len()
}
