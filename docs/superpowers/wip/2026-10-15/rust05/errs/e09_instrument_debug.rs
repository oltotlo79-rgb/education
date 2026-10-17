use tracing::instrument;

// Debug を実装していない型
struct Password(String);

#[instrument]
async fn login(email: &str, password: Password) -> bool {
    !password.0.is_empty() && !email.is_empty()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    login("a@example.com", Password(String::from("secret"))).await;
}
