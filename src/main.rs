#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate select;

mod helpers;

use helpers::get_base_url;
//use reqwest::StatusCode;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use std::{io, process, thread, time};
use url::{ParseError, Url};

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

fn main() -> Result<()> {
    let response: reqwest::Response = loop {
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
    };

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

    println!("Finding links for \"{}\"...", response.url().as_str());

    let url = Url::parse(response.url().as_str()).unwrap();
    let document = Document::from_read(response);
    let document = match document {
        Ok(doc) => doc,
        Err(_) => {
            println!("Error reading website. It is UTF-8 encoded? Please try another site..."); // google.com issue
            process::exit(1);
        }
    };

    let base_url = get_base_url(&url, &document);

    let mut links = HashSet::new();
    let mut count = 0;

    document
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|link| {
            match Url::parse(link) {
                Ok(url) => {
                    count += 1;
                    links.insert(url);
                }
                Err(ParseError::RelativeUrlWithoutBase) => {
                    links.insert(base_url.join(link).unwrap());
                }
                Err(e) => println!("Could not parse '{}'.", e),
            };
        });
    /*
        let mut broken = 0;
        for link in &links {
            let res = reqwest::get(link).expect("Error reading link.");
            let doc = Document::from_read(res)?;

            match res.status() {
                StatusCode::NOT_FOUND => {
                    broken += 1;
                }
                _ => (),
            };
            println!("{}", link);
        }
    */
    println!(
        "\nFound {} links, {} of which are unique.",
        count,
        links.len(),
    );

    Ok(())
}
