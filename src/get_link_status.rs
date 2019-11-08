extern crate select;
extern crate term;
extern crate url;

use reqwest::StatusCode;
use std::{
    collections::HashSet,
    io::{prelude::*, stdout},
};
use term::color::{BRIGHT_BLACK, CYAN, MAGENTA};

pub fn get_link_status(links: HashSet<String>) -> (usize, usize, usize, usize) {
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

    if !good_links.is_empty() {
        println!("\nThe following pages were found:");
        for good_link in &good_links {
            print!("‣ ");
            t.fg(CYAN).unwrap();
            writeln!(t, "{}", good_link).unwrap();
            t.reset().unwrap();
        }
    }

    if !broken_links.is_empty() {
        println!("\nThe following pages were not found or cause redirects:");
        for broken_link in &broken_links {
            print!("‣ ");
            t.fg(MAGENTA).unwrap();
            writeln!(t, "{}", broken_link).unwrap();
            t.reset().unwrap();
        }
    }

    if !not_links.is_empty() {
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

#[cfg(test)]
mod tests {
    // TODO: Write tests. How to mock responses to reqwest within this function?
    // Simple test page: https://charmedsatyr.github.io/document/

    #[test]
    fn get_link_status_should_return_correct_number_of_total_links_in_hashset() {
        //
    }

    #[test]
    fn get_link_status_should_return_correct_number_of_good_links_in_hashset() {
        //
    }

    #[test]
    fn get_link_status_should_return_correct_number_of_broken_links_in_hashset() {
        //
    }

    #[test]
    fn get_link_status_should_return_correct_number_of_not_links_in_hashset() {
        //
    }
}
