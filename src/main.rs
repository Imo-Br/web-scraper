use web_scraper::site::get_html;
use web_scraper::HtmlTag;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>> {
    let file = get_html("https://rust-lang.org").await.unwrap();
    // Define the tag you want to parse
    let tag = HtmlTag::DIV;
    // Parse the <div> tags and collect the results into a vector
    let new_vector = tag.parse_tags(&file);

    // Print the parsed <div> tags
    for li_tag in &new_vector {
        println!("{}", li_tag);
    }

    Ok(())
}
