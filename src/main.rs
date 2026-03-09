use reqwest::Client;
use reqwest::header::{HeaderMap, USER_AGENT};
use std::error::Error;
use std::time::Instant;

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

    let first_time = Instant::now();
    let response: reqwest::Response = client
        .post("http://localhost:8080/wp-login.php")
        .headers(headers)
        .form(&params)
        .send()
        .await?;

    let success: bool = response.url().path().contains("wp-admin");
    let duration: f64 = first_time.elapsed().as_secs_f64() * 1000.0;

    Ok(format!("{} {}", success, duration))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    for i in 1..50 {
        let result = wp_login("admin", "123456").await?;
        println!("{:?}", result);
    }
    Ok(())
}
