use anonservice_api::utils::{get_content, get_elements_on_page, get_max_page};
use reqwest::Error;
use scraper::Html;
// use crate::utils::{get_content, get_elements_on_page, get_max_page};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let content = get_content("").await?;

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
