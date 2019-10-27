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
    color::{BRIGHT_BLACK, CYAN, MAGENTA},
    Attr::Bold,
};
use url::{ParseError, Position, Url};

fn add_protocol(trimmed_entry: &str) -> String {
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

pub fn handle_entry() -> Response {
    loop {
        let mut t = term::stdout().unwrap();

        print!("Enter a URL to scrape for links, e.g., ");
        t.fg(CYAN).unwrap();
        write!(t, "ma.tt").unwrap();
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
        "https://streamate.com/"
        | "https://www.streamate.com/"
        | "https://www.jerkmatelive.com/"
        | "https://www.heidismembersonlyclub.com/"
        | "https://www.youpornlive.com/"
        | "https://www.pornhublive.com/"
        | "https://www.pornhub.com/"
        | "https://www.youporn.com/" => {
            println!("\nPorn? You hound!");
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

pub fn get_links(document: &Document, base_url: &Url) -> (HashSet<String>) {
    let mut links = HashSet::new();

    document
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|link| {
            match Url::parse(link) {
                Ok(url) => {
                    links.insert(url.as_str().to_owned());
                }
                Err(ParseError::RelativeUrlWithoutBase) => {
                    let l = base_url
                        .join(link)
                        .expect("Trouble resolving relative URL.");
                    links.insert(l.as_str().to_owned());
                }
                Err(e) => println!("Could not parse \"{}\".", e),
            };
        });

    if links.len() == 0 {
        println!("No links were found on that page.");
        process::exit(0);
    } else {
        println!("{} links found...", links.len());
    }

    links
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

pub fn check_link_status(links: HashSet<String>) -> (usize, usize, usize, usize) {
    print!("\nValidating links...");
    stdout().flush().unwrap();

    let mut good_links = HashSet::new();
    let mut broken_links = HashSet::new();
    let mut not_links = HashSet::new();
    for link in &links {
        let res = reqwest::get(link);

        match res {
            Ok(r) => {
                if r.status() == StatusCode::NOT_FOUND {
                    broken_links.insert(link);
                } else {
                    good_links.insert(link);
                };
            }
            Err(_) => {
                not_links.insert(link);
                continue;
            }
        }
    }
    println!(" Done.");

    let mut t = term::stdout().unwrap();

    if good_links.len() > 0 {
        println!("\nThe following pages were found:");
        for good_link in &good_links {
            print!("‣ ");
            t.fg(CYAN).unwrap();
            writeln!(t, "{}", good_link).unwrap();
            t.reset().unwrap();
        }
    }

    if broken_links.len() > 0 {
        println!("\nThe following pages were not found or cause redirects:");
        for broken_link in &broken_links {
            print!("‣ ");
            t.fg(MAGENTA).unwrap();
            writeln!(t, "{}", broken_link).unwrap();
            t.reset().unwrap();
        }
    }

    if not_links.len() > 0 {
        println!("\nThe following returned invalid status codes or are not URLs:");
        for not_link in &not_links {
            print!("‣ ");
            t.fg(BRIGHT_BLACK).unwrap();
            writeln!(t, "{}", not_link).unwrap();
            t.reset().unwrap();
        }
    }

    (
        links.len(),
        good_links.len(),
        broken_links.len(),
        not_links.len(),
    )
}
