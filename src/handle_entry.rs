use super::add_protocol::*;
use reqwest::{get, Response};
use std::{io::stdin, process};
use term::{color::CYAN, Attr::Bold};

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
        let result_from_initial_get_request_to_entry = get(&trimmed_entry);

        match result_from_initial_get_request_to_entry {
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

#[cfg(test)]
mod tests {
    // TODO: Write tests. How to mock stdin and responses to reqwest within this function?
    // Simple test page: https://charmedsatyr.github.io/document/

    #[test]
    fn handle_entry_should_return_response_from_valid_url_input() {
        //
    }

    #[test]
    fn handle_entry_should_exit_on_q_input() {
        //
    }

    #[test]
    fn handle_entry_should_loop_on_invalid_url_input() {
        //
    }
}
