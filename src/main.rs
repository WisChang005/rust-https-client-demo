use hyper::Client;
use hyper_tls::HttpsConnector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client
        .get("https://currency-exchange.p.rapidapi.com/exchange".parse()?)
        .await?;
    assert_eq!(res.status(), 200);
    Ok(())
}
