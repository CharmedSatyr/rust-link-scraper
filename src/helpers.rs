extern crate select;
extern crate term;
extern crate url;

use reqwest::{Response, StatusCode};
use select::{document::Document, predicate::Name};
use std::{
    collections::HashSet,
    io::{prelude::*, stdin, stdout},
    process, thread, time,
};
use term::{
    color::{CYAN, MAGENTA},
    Attr::Bold,
};
use url::{ParseError, Position, Url};

fn add_protocol(trimmed_entry: &str) -> String {
    let insecure = String::from("http://");
    let mut with_protocol = String::from("https://");

    let mut result = trimmed_entry;
    if trimmed_entry[..8] != with_protocol && trimmed_entry[..7] != insecure {
        with_protocol.push_str(trimmed_entry);
        result = &with_protocol;
    }
    String::from(result)
}

pub fn handle_entry() -> Response {
    loop {
        let mut t = term::stdout().unwrap();

        print!("Enter a URL to scrape for links, e.g., ");
        t.fg(CYAN).unwrap();
        write!(t, "www.charmed.tech").unwrap();
        t.reset().unwrap();
        print!(", or enter ");
        t.attr(Bold).unwrap();
        write!(t, "q").unwrap();
        t.reset().unwrap();
        println!(" to quit.");

        let mut entry = String::new();

        stdin()
            .read_line(&mut entry)
            .expect("Failed to read input!");

        let trimmed_entry = entry.trim();

        if trimmed_entry == "q" {
            process::exit(0);
        }

        let trimmed_entry = add_protocol(trimmed_entry);
        let result = reqwest::get(&trimmed_entry);

        match result {
            Ok(response) => {
                break response;
            }
            Err(e) => {
                println!("Invalid entry! Error: {}.\n", e);
                continue;
            }
        };
    }
}

pub fn joke(response: &Response) {
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

pub fn get_response(response: Response) -> (Url, Document) {
    let mut t = term::stdout().unwrap();
    print!("\nReading ");
    t.fg(CYAN).unwrap();
    write!(t, "{}", response.url().as_str()).unwrap();
    t.reset().unwrap();
    println!("...");

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
    let mut t = term::stdout().unwrap();
    println!("\nThe following unique URLs were found:");

    let mut count = 0;
    for link in links {
        count += 1;
        write!(t, "‣ ").unwrap();
        t.fg(CYAN).unwrap();
        println!("{}", link);
        t.reset().unwrap();
    }
    count
}

pub fn find_broken_links(links: &HashSet<String>) -> usize {
    print!("\nLooking for broken links...");
    stdout().flush().unwrap();

    let mut broken_links = HashSet::new();
    for link in links {
        let res = reqwest::get(link); //.expect("\nError reading link.");

        match res {
            Ok(r) => {
                if r.status() == StatusCode::NOT_FOUND {
                    broken_links.insert(link);
                };
            }
            Err(_) => continue, // Likely an email address or script element
        }
    }
    println!(" Done.");

    if broken_links.len() > 0 {
        let mut t = term::stdout().unwrap();
        println!("\nAmong the unique URLs, the following pages are broken or cause redirects:");
        for broken in &broken_links {
            write!(t, "‣ ").unwrap();
            t.fg(MAGENTA).unwrap();
            writeln!(t, "{}", broken).unwrap();
            t.reset().unwrap();
        }
    }

    broken_links.len()
}
