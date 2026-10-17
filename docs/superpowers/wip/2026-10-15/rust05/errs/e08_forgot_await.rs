#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    // send() の後の .await を忘れた
    let response = client.get("http://127.0.0.1:3000/health").send();
    println!("{}", response.status());
}
