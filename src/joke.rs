use reqwest::Response;
use std::{thread, time};

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
