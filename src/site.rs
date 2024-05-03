/// Get the HTML content of a website to then scrape the content in it
///
/// # Arguments
///
/// * `url` - The url of the website
///
/// # Returns
///
/// * `Ok(String)` - The html content of the website
/// * `Err(Box<dyn std::error::Error>)` - An error if the request fails or the content type is not text/html
///
/// # Examples
///
/// ```
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>> {
/// use web_scraper::site::get_html;
/// use web_scraper::HtmlTag;
/// let url = "https://example.com";
/// let html = get_html(url).await.unwrap();
/// let tag = HtmlTag::DIV;
/// // Parse the <div> tags and collect the results into a vector of strings
/// let new_vector = tag.parse_tags(&html);
/// Ok(())
/// }
/// ```

pub async fn get_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = match client.get(url).send().await {
        Ok(response) => response,
        Err(e) => {
            println!("Error: {}\n", e);
            return Err(
                "Request failed, check if the url is valid\nFormat: https://example.com\nAlso check if the domain actually exists".to_string().into(),
            );
        }
    };

    let html = response.text().await?;

    Ok(html)
}
