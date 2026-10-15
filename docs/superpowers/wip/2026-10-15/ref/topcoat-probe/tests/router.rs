//! Integration tests that call the router directly with `Router::handle`
//! (no TCP socket, no reqwest). Run: cargo test

use topcoat::router::{
    Body, Method, StatusCode, header, request::Request, response::Response, to_bytes,
};

async fn body_string(response: Response) -> String {
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}

fn get(uri: &str) -> Request {
    Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Body::empty())
        .unwrap()
}

#[tokio::test]
async fn home_page_renders() {
    let router = topcoat_probe::app::router();
    let response = router.handle(get("/")).await;
    assert_eq!(response.status(), StatusCode::OK);
    let html = body_string(response).await;
    assert!(html.contains("Not signed in"), "{html}");
}

#[tokio::test]
async fn health_is_json() {
    let router = topcoat_probe::app::router();
    let response = router.handle(get("/api/health")).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()[header::CONTENT_TYPE], "application/json");
    assert_eq!(body_string(response).await, r#"{"status":"ok"}"#);
}

#[tokio::test]
async fn login_then_create_note_then_read_it() {
    let router = topcoat_probe::app::router();

    // POST /login (form) -> 303 + Set-Cookie
    let login = Request::builder()
        .method(Method::POST)
        .uri("/login")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from("name=alice&password=secret"))
        .unwrap();
    let response = router.handle(login).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    let set_cookie = response.headers()[header::SET_COOKIE].to_str().unwrap();
    let cookie = set_cookie.split(';').next().unwrap().to_owned(); // "__Host-session=..."

    // POST /notes with the session cookie -> 303 Location: /notes/1
    let create = Request::builder()
        .method(Method::POST)
        .uri("/notes")
        .header(header::COOKIE, &cookie)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from("title=Hello&body=World"))
        .unwrap();
    let response = router.handle(create).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(response.headers()[header::LOCATION], "/notes/1");

    // GET /notes/1
    let show = Request::builder()
        .uri("/notes/1")
        .header(header::COOKIE, &cookie)
        .body(Body::empty())
        .unwrap();
    let response = router.handle(show).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert!(body_string(response).await.contains("<h1>Hello</h1>"));
}

#[tokio::test]
async fn unknown_url_is_404() {
    let router = topcoat_probe::app::router();
    let response = router.handle(get("/nope")).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
