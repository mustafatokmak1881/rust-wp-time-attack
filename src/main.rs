use reqwest::Client;
use reqwest::header::{HeaderMap, USER_AGENT};
use std::error::Error;

async fn wp_login(log: &str, pwd: &str) -> Result<String, Box<dyn Error>> {
    let client: Client = Client::builder().cookie_store(true).build()?;

    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(USER_AGENT, "Test".parse()?);

    let params: [(&str, &str); 5] = [
        ("log", log),
        ("pwd", pwd),
        ("wp_submit", "Oturum Ac"),
        ("redirect_to", "http://localhost:8080/wp-admin"),
        ("test_cookies", "1"),
    ];

    let response: reqwest::Response = client
        .post("http://localhost:8080/wp-login.php")
        .headers(headers)
        .form(&params)
        .send()
        .await?;

    Ok(response.url().to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let result = wp_login("admin", "123456").await?;
    println!("Result: {:?}", result);

    Ok(())
}
