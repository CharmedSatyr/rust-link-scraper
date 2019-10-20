#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use std::{io, thread, time};

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

fn main() -> Result<()> {
    let response: reqwest::Response = loop {
        println!("Enter a full url to scrape for links, e.g., \"https://www.nytimes.com\":");

        let mut entry = String::new();

        io::stdin()
            .read_line(&mut entry)
            .expect("Failed to read input!");

        println!("Reading input...");

        let trimmed_entry = entry.trim();

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

    let valid_url = response.url().as_str();

    match valid_url {
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

    println!("Finding links for \"{}\"...", valid_url);

    let mut links = HashSet::new();
    let mut count = 0;

    let document = Document::from_read(response);

    match document {
        Ok(doc) => doc
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .for_each(|link| {
                count += 1;
                links.insert(link.to_string());
            }),
        Err(_) => println!("Error reading the website. This program requires UTF-8 encoding."),
    }
    for link in &links {
        println!("{}", link);
    }

    println!(
        "\nFound {} links, {} of which are unique.",
        count,
        links.len()
    );

    Ok(())
}
