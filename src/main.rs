use reqwest::{self, Error};
use scraper::{Html, Selector};

const BASE_URL: &str = "https://anon.services";

#[derive(Debug)]
struct FileInfo {
    name: String,
    link: String,
}

fn search(query: &str, page: &str) -> String {
    format!("{BASE_URL}/search?q={}&page={}&sort=time_desc", query, page)
}

async fn get_initial_page_content() -> Result<String, Error> {
    let client = reqwest::Client::new();
    let resp = client.get(search("", "1")).send().await?.text().await?;
    Ok(resp)
}

fn get_max_page(document: &Html) -> u32 {
    let selector = Selector::parse("main.container > p:nth-child(4)").unwrap();
    document
        .select(&selector)
        .next()
        .and_then(|element| {
            let text = element.text().collect::<String>();
            text.split(" of ")
                .last()
                .and_then(|pages_str| {
                    pages_str
                        .trim()
                        .chars()
                        .take_while(|char| char.is_ascii_digit())
                        .collect::<String>()
                        .parse::<u32>()
                        .ok()
                })
        })
        .unwrap_or(0)
}

fn get_elements_on_page(document: &Html) -> Vec<FileInfo> {
    let mut results = Vec::new();
    let row_selector = Selector::parse("table.files-list > tbody > tr").unwrap();
    let link_selector = Selector::parse("td.ohidden.breaktext > a.decoration-none").unwrap();

    for row_element in document.select(&row_selector) { // тут вообще пиздец чуток, но ладно, иди нахуй
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
    let content = get_initial_page_content().await?;
    let doc = Html::parse_document(&content);

    let max_pages = get_max_page(&doc);
    println!("{}", max_pages);

    let files = get_elements_on_page(&doc);
    for (index, file_info) in files.iter().enumerate() {
        println!( // ыыыы ванючы индекс ыыыы плаки плаки блять, почему \t?!!!
            "Index: {}\tName: {}, Link: {}",
            index + 1,
            file_info.name,
            file_info.link
        );
    }

    Ok(())
}
