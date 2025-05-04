use anonservice_api::utils::{get_content, get_max_page};
use scraper::Html;

#[tokio::test]
async fn test_max_pages_with_unknown_name() {
    let content = get_content("ttt").await.unwrap();

    let doc = Html::parse_document(&content);
    assert_eq!(get_max_page(&doc), 0);
}

#[tokio::test]
async fn test_max_pages() {
    let content = get_content("windows xp").await.unwrap();

    let doc = Html::parse_document(&content);
    assert_ne!(get_max_page(&doc), 0);
}
