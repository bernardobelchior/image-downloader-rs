extern crate reqwest;

use std::path::PathBuf;
use std::fs::File;
use reqwest::Response;

fn main() {
    const BASE_URL: &'static str = "https://livebooks.klett-sprachen.de/978-3-12-606142-1/preview/big/";
    const EXTENSION: &'static str = ".jpg";
    const START: u32 = 1;
    const END: u32 = 173;
    const DOWNLOAD_PATH: [&'static str; 2] = ["download", "a21"];

    let path: PathBuf = DOWNLOAD_PATH.iter().collect();

    for page_no in START..END {
        match request(BASE_URL, page_no, EXTENSION) {
            Some(mut content) => {
                save(&path, page_no, EXTENSION, &mut content);
            }
            _ => ()
        }
    }
}

fn request<'a, 'b>(base_url: &'a str, page_no: u32, extension: &'b str) -> Option<Response> {
    let mut url = base_url.to_owned();
    url.push_str(&page_no.to_string());
    url.push_str(extension);

    let resp = reqwest::get(&url);

    if let Ok(body) = resp {
        return Some(body);
    }

    None
}

fn save<'a>(base_path: &PathBuf, page_no: u32, extension: &'a str, content: &mut Response) {
    let mut path = base_path.clone();
    path.push(page_no.to_string() + extension);

    let mut file = File::create(path).unwrap();

    content.copy_to(&mut file).expect("Error writing to file");
}
