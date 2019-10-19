#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;
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

        let mut url = String::new();
        io::stdin()
            .read_line(&mut url)
            .expect("Failed to read input!");

        println!("Reading input...");

        let result = reqwest::get(&url);
        match result {
            Ok(r) => {
                break r;
            }
            Err(e) => {
                println!("Invalid URL! Error: {}.", e);
                continue;
            }
        };
    };

    // Joke
    let sec = time::Duration::from_millis(1000);
    match response.url().as_str() {
        "https://www.streamate.com/"
        | "https://www.jerkmatelive.com/"
        | "https://www.youporn.com/"
        | "https://www.pornhub.com/" => {
            println!("Porn? You hound!");
            thread::sleep(sec);
        }
        "https://www.google.com/" => {
            println!("This site has an encoding I can't handle right now.");
            thread::sleep(sec);
        }
        _ => (),
    };

    println!("Finding links for \"{}\"...", response.url().as_str());

    Document::from_read(response)
        .expect("Cannot read response...")
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x)); // Add to a vector
                                          // If vector has len 0, message
    Ok(())
}
