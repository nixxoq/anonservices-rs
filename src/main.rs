use reqwest::{self, Error};
use scraper::{Html, Selector};

const BASE_URL: &'static str = "https://anon.services";

#[derive(Debug)]
struct FileInfo {
    name: String,
    link: String,
}

fn search(query: &str, page: &str) -> String {
    format!("{BASE_URL}/search?q={}&page={}&sort=time_desc", query, page)
}

async fn get_content() -> Result<String, Error> {
    let client = reqwest::Client::new();

    let resp = client.get(search("", "1")).send().await?.text().await?;

    Ok(resp)
}

fn get_max_page(document: &Html) -> u32 {
    let selector = Selector::parse("main.container > p:nth-child(4)").unwrap();
    let mut max_pages: u32 = 0;

    document
        .select(&selector)
        .next()
        .iter()
        .for_each(|element| {
            let text = element.text().collect::<String>();
            if let Some(pages_str) = text.split(" of ").last() {
                if let Ok(pages) = pages_str
                    .trim()
                    .chars()
                    .take_while(|char| char.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u32>()
                {
                    max_pages = pages;
                }
            }
        });
    max_pages
}

fn get_elements_on_page(document: &Html) -> Vec<FileInfo> {
    let mut results = Vec::new();
    let row_selector = Selector::parse("table.files-list > tbody > tr").unwrap();
    let link_selector = Selector::parse("td.ohidden.breaktext > a.decoration-none").unwrap();

    for row_element in document.select(&row_selector) {
        if let Some(link_element) = row_element.select(&link_selector).next() {
            let name = link_element.text().collect::<String>().trim().to_string();
            if let Some(relative_link) = link_element.value().attr("href") {
                let full_link = format!("{}{}", BASE_URL, relative_link);
                results.push(FileInfo {
                    name,
                    link: full_link,
                });
            }
        }
    }
    results
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let content = get_content().await?;

    let doc = Html::parse_document(&content);

    println!("{}", get_max_page(&doc));
    get_elements_on_page(&doc)
        .iter()
        .enumerate()
        .for_each(|(index, file_info)| {
            println!(
                "Index: {}\tName: {}, Link: {}",
                index + 1,
                file_info.name,
                file_info.link
            )
        });
    Ok(())
}
