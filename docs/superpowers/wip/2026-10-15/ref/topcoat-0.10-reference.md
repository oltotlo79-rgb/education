# Topcoat 0.10 verified reference (for the RustNote textbook)

Status: **partial, work stopped early by the coordinator** (2026-10-15). Everything marked VERIFIED was compiled and/or run on this machine (Windows 11 ARM64, MSVC). Items marked UNVERIFIED come from the crate's own docs or source only. The last section lists what was not investigated.

Sources used: crate sources under `~/.cargo/registry/src/*/topcoat-*-0.10.0` (most authoritative), the GitHub repo at tag `v0.10.0` (examples, `llms.txt`, docs), crates.io API, and the old 0.5.0 and 0.9.0 crate sources for the diff. I did **not** read docs.rs HTML or the tokio.rs announcement blog post, because the crate sources contain the same docs (`#![doc = include_str!(...)]`).

---

## 0. Versions and install (VERIFIED)

| Item | Value |
|---|---|
| topcoat | **0.10.0**, the newest 0.10.x as of 2026-10-15 (released 2026-10-03; there is no 0.10.1). Pin with `topcoat = "=0.10.0"` |
| MSRV | **Rust 1.98** (`rust-version = "1.98"`). **rustc 1.97.1 cannot build it**, not even with `--ignore-rust-version` (error `E0658 int_format_into` in topcoat-view). Current stable is 1.99.0 |
| topcoat-cli | **0.10.0** (MSRV 1.98). Installs two binaries: `topcoat` and `cargo-topcoat` |
| tokio | 1.53.3 resolved (`tokio = { version = "1", features = ["rt-multi-thread", "macros"] }` is enough) |
| serde | 1.0.229 (`features = ["derive"]`) |
| libsql | **0.10.0** (released 2026-10-13) |
| argon2 | **0.6.0** (password-hash 0.6.1). **Its API differs from 0.5**, see §12 |

What I installed on this machine: `rustup toolchain install 1.99.0 --profile minimal --component rustfmt,clippy`. This adds a toolchain next to the default without changing it. Side effect: rustup updated itself to 1.29.1. Each probe pins the toolchain with a `rust-toolchain.toml` containing `channel = "1.99.0"`. For the textbook, readers should simply run `rustup update` (stable 1.99 or newer).

```sh
cargo new rustnote && cd rustnote
cargo add topcoat@=0.10.0
cargo add tokio --features rt-multi-thread,macros
cargo add serde --features derive
cargo install topcoat-cli --version 0.10.0 --locked   # took about 2 min here; works on Windows ARM64
```

### CLI commands (VERIFIED with `topcoat --help` and by running them)

- `topcoat dev [--bin B] [-p P] [-r|--release] [--profile NAME]` builds, bundles assets, runs the app and rebuilds on change.
  - Verified on Windows: after editing a .rs file the page updated after about 5 s.
  - It has no `--port` flag. Set the `HOST` and `PORT` environment variables instead.
  - It injects `TOPCOAT_DEV_URL`. `topcoat::dev::script()` then renders `<script src="http://127.0.0.1:<random>/dev.js">`. Outside `topcoat dev` it renders nothing (verified).
  - The running child process is a copy named `<bin>.topcoat-dev` (for example `topcoat-probe.topcoat-dev`). If you kill `topcoat.exe` forcibly, that child can survive and keep holding the port. Stop the dev server with Ctrl+C.
  - `cargo topcoat dev` also works (documented; the binary was installed).
- `topcoat fmt [FILES] [--check] [--rustfmt] [--stdin] [--macros a,b]` formats `view!` and other macro bodies.
  - Verified: `--check` exits 1 when files need formatting.
  - `--rustfmt` needs `rustfmt` on PATH and a `rustfmt.toml` with `edition = "2024"`.
  - All probe files were formatted with it and still compile.
- `topcoat asset list | bundle | clean`. `bundle` also takes `--bin/-p/--release/--profile` and `-o/--out DIR`.
- `topcoat ui init | add | list | remove` exists (not tried).
- **There is no `topcoat new` in 0.10.0.** It was added on `main` after the release (#501, 2026-10-08) and is unreleased.

---

## 1. Cargo.toml and feature flags (VERIFIED)

The default features are `asset, compression, cookie, font, icon, router, runtime, serve, session, view, discover`. They cover everything RustNote needs. Optional features: `tailwind`, `fs` (`public_dir`/`serve_dir`), `multipart`, `sse`, `websocket`, `tower`, `htmx`, `alpine-ajax`, `datastar`, `mail`, `mail-smtp`, `sitemap`, `anyhow`, `ui`, `icon-iconify`, `font-fontsource`, `full`.

```toml
[package]
name = "rustnote"
version = "0.1.0"
edition = "2024"
default-run = "rustnote"   # only needed if you add more binaries

[dependencies]
topcoat = "=0.10.0"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
serde = { version = "1", features = ["derive"] }
libsql = "=0.10.0"
argon2 = "=0.6.0"
```

Topcoat itself is pure Rust: `cargo tree` shows no `cc` and no `ring` (VERIFIED).

---

## 2. `main`, `start`, HOST/PORT (VERIFIED: ran + curl)

`topcoat::start(router)` reads **`HOST` and `PORT` from the environment** and defaults to `127.0.0.1:3000`. It is not configured through the builder.

- Fly.io needs `HOST=0.0.0.0 PORT=8080`. Verified: `HOST=0.0.0.0 PORT=8089` listened on `0.0.0.0:8089` and served requests.
- An invalid `PORT` (for example `abc`) makes `start` return `Err(InvalidInput "PORT must be a valid port number...")`.
- Graceful shutdown happens on Ctrl+C (and SIGTERM on Unix).
- Other entry points: `topcoat::serve(listener, router)` and `serve_until(listener, router, signal)`.

```rust
// src/main.rs  (probe: topcoat-probe/src/main.rs)
#[tokio::main]
async fn main() {
    topcoat::start(rustnote::app::router()).await.unwrap();
}
```

```rust
// Explicit-path style (probe: examples/hello.rs)
use topcoat::router::{Router, RouterBuilderDiscoverExt}; // the trait import is REQUIRED for .discover()
let router = Router::builder().discover().build();
```

Module-routing style, which the docs recommend: call `module_router!()` in the root module of the route tree. That module maps to `/`. It returns a `RouterBuilder`, so you chain `.cookies()`, `.sessions(..)`, `.app_context(..)`, `.discover()`, `.assets(..)` and then `.build()`.

Windows gotcha (observed): if another process already listens on `0.0.0.0:3000` (here, a node.exe), binding `127.0.0.1:3000` still succeeds, and requests may go to the other process. If you see an unexpected page, choose another port.

---

## 3. Routing (VERIFIED: ran + curl, unless marked)

Full working app: `topcoat-probe/src/app.rs` and its submodules.

### Module routing

- `module_router!()` does not scan the filesystem. Every module must be declared with `mod`.
- Each module adds one kebab-case segment (`blog_posts` → `/blog-posts`).
- Modules whose names start with `_` are groups: they add no URL segment but still scope layouts and layers.
- The function name does not affect the URL.
- Several handlers in one module share the path, but may use different methods.

```
src/app.rs                    -> GET /   (+ #[layout], #[layer], not_found!())
src/app/login.rs              -> #[page] GET /login + #[page(POST)] POST /login
src/app/notes.rs              -> #[page] GET /notes + #[route(POST)] POST /notes
src/app/notes/new.rs          -> GET /notes/new   (the static segment beats {note_id}: verified)
src/app/notes/note_id.rs      -> module_param!(note_id: u64, error = not_found) => GET /notes/{note_id}
                                 #[route(POST "./delete")] => POST /notes/{note_id}/delete
src/app/notes/note_id/edit.rs -> GET + POST /notes/{note_id}/edit
src/app/api/health.rs         -> #[route(GET)] GET /api/health
```

### Handler forms

| Macro | Signature and notes |
|---|---|
| `#[page]` | `async fn x(cx: &Cx, <one FromRequest body>) -> Result<impl View>`. Both parameters are optional, in any order. GET by default. `#[page(POST)]`, `#[page([GET, POST])]` and `#[page(*)]` are allowed |
| `#[route(GET)]` | Method is required: `#[route(POST)]`, `#[route([GET, POST])]`, `#[route(*)]`. Optional path: `#[route(GET "./sub")]` (relative) or `"/abs"` |
| `#[layout]` | `async fn l(cx: &Cx, slot: Slot<'_>) -> Result<impl View>`. Render the inner page with `(slot)` |
| `#[layer]` | `async fn m(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response>`. Call `next.run(cx, body).await`. VERIFIED (access-log layer) |

- An absolute path such as `#[page("/x")]` is ignored by `module_router!`. Register it with `.page(x)` or `.route(x)`, or call `.discover()`.
- A wrong method returns 405 with an `allow: GET, POST, HEAD` header.

### Path parameters (0.10 semantics, VERIFIED with `examples/path_param_gotcha.rs`)

- `module_param!(note_id: u64, error = not_found)` generates `pub? struct NoteId(u64)` and **sets this module's URL segment to `{note_id}`** (the file name does not matter).
- `path_param!(post_id: u64)` only declares the type. **The module keeps its static segment.**
  - Verified: `path_param!` inside `mod legacy::id` serves the literal URL `/legacy/id`, and `/legacy/7` returns 404.
  - Use it with an explicit placeholder: `#[page("./{post_id}")]` or `#[page("/users/{user_id}")]`.
- Read a parameter with `path_param::<NoteId>(cx)`:
  - With `error = ...`, the result converts via `?` into a router error (404 for `not_found`, 400 for `bad_request` with the body `bad request: invalid value for path parameter "post_id"`).
  - Without `error`, it returns `Result<&T, &<T as FromStr>::Err>`.
  - Without a type (`path_param!(slug)`), it returns `&str` and cannot fail.
- Allowed `error =` values: `not_found`, `unauthorized`, `forbidden`, `bad_request`, `bad_request("msg")`, `redirect("/p")`, `redirect_permanent("/p")`.
- Visibility: `module_param!(pub(crate) note_id: u64, error = not_found);`
- Catch-all parameters: `module_param!(*path)` or `path_param!(*ids: u32)` (UNVERIFIED).

### Links: `href!` (VERIFIED)

```rust
<a href=(href!(note_id::show, note_id::NoteId(note.id)))>(note.title)</a>
<form method="post" action=(href!(super::create))>...</form>
see_other(href!(super::index).resolve(cx))   // .resolve(cx) -> String
```

Also available: `.query(serializable)`, `.fragment("x")`, `.is_current(cx)` (UNVERIFIED).

### Query parameters (VERIFIED)

```rust
#[query_params(error = bad_request)]   // derives Deserialize; struct with named fields
pub(crate) struct NotesQuery { q: Option<String> }
let query = query_params::<NotesQuery>(cx)?;   // &NotesQuery, memoized per request
```

Both `?q=` (empty value) and a missing key read as `None` (verified).

### 404 handling (VERIFIED)

- `not_found!();` in the root module registers a catch-all page. Every unmatched URL then yields a `NotFoundError` that flows through the layouts.
- The root layout wraps `(slot)` in an `error_boundary` and renders a branded 404 with `(StatusCode::NOT_FOUND)`. See `src/app.rs`.
- Without `not_found!()`, unmatched URLs get a plain-text 404 (`not found`) and no layout runs.

---

## 4. `view!` syntax (VERIFIED: ran + curl, `examples/view_syntax.rs`)

Rendered HTML confirmed.

- **Text must be quoted**: `<p>"Hello, " (name) "!"</p>`. Expressions go in `( )`.
- **Interpolated values are HTML-escaped**:
  - `"<b>Ada</b>"` renders as `&lt;b&gt;Ada&lt;/b&gt;`.
  - A note body `<script>alert(1)</script> & more` renders as `&lt;script&gt;alert(1)&lt;/script&gt; &amp; more`.
  - To opt out with trusted HTML only, use `topcoat::view::Unescaped::new_unchecked("<em>..</em>")`.
- Void elements have no closing tag (`<input ...>`, `<br>`, `<meta ...>`). `<!DOCTYPE html>` is allowed.
- Control flow works in children: `if`/`else`, `if let`, `match`, `for`, and `let x = ...;`.
  - A `match` arm that produces several nodes needs `{ ... }`.
  - `_ => ""` renders nothing.
  - `#[key(item.id)] for item in items { ... }` sets an optional key.
- Control flow also works inside the attribute list: `<a href="/posts" if logged_in { aria-current="page" class="active" }>`.
- Attribute values:
  - `disabled=(false)` omits the attribute; `true` renders an empty attribute.
  - `title=(None::<&str>)` omits the attribute.
  - `required=""` always renders.
- `attributes! { id="save" type="submit" data-x=(42) }` collects attributes; spread them with `<button (attrs)>`.
- `class!("box", "box--on" if on, extra_opt)` joins entries and skips absent ones.
- Components:
  - Define with `#[component] async fn card(title: &str, #[default] highlighted: bool, #[default] child: Child<'_>) -> Result<impl View>`.
  - Call with `card(title: "Welcome", highlighted: true, <p>"child"</p>)`: named arguments first, then children without commas.
  - Parameter attributes: `#[default]`, `#[default(expr)]`, `#[into]`.
  - Add `cx: &Cx` as a parameter and it is injected automatically.
- Response metadata inside a view:
  - `(StatusCode::IM_A_TEAPOT)` sets the status (verified 418).
  - `((header::CACHE_CONTROL, HeaderValue::from_static("no-store")))` adds a header (verified).
- **Gotcha (VERIFIED compile error):** an opaque `impl View` returned from a plain function cannot be interpolated directly. Use `(helper(cx).await?.boxed())` with `topcoat::view::ViewExt`. A local `let v = view!{...}` can be interpolated as `(v)`.
- A plain (non-component) function renders a view as `view! { cx => badge(label: "x") }`.

---

## 5. `Cx`, forms, JSON, redirects, errors (VERIFIED: ran + curl)

### Request helpers

`use topcoat::router::request::{headers, method, uri, parts, content_type, client_ip, remote_addr, extensions};`, then call `headers(cx).get("user-agent")` and so on.

### HTML form into a struct

```rust
#[derive(serde::Deserialize)]
pub(crate) struct NoteForm { pub title: String, pub body: String }

#[route(POST)]
pub(crate) async fn create(cx: &Cx, Form(form): Form<NoteForm>) -> Result<SeeOther> {
    let user = require_user(cx).await?;
    if form.title.trim().is_empty() { return Err(bad_request("title is required").into()); }
    let id = db(cx).create_note(&user.name, form.title, form.body);
    Ok(see_other(href!(note_id::show, note_id::NoteId(id)).resolve(cx)))   // 303
}
```

- `Form` lives at `topcoat::router::content::Form`.
- For POST it requires `Content-Type: application/x-www-form-urlencoded`. Another content type returns 400.
- A missing field returns 400 with the body ``bad request: invalid form value: missing field `body` (at `.`)``.
- For GET, `Form<T>` reads the query string.
- The default body limit is 2 MiB (`BodyLimit`, UNVERIFIED).

### Redirect from a page

A page's `Ok` value is a view, so a redirect leaves through `Err`: `return Err(see_other(url).into());`. Verified 303 in `edit.rs` and `login.rs`.

### Redirect constructors

- `see_other(u)` → 303. Use it after POST.
- `redirect(u)` → **307**.
- `redirect_permanent(u)` → 308.

All live in `topcoat::router::error`.

### Error constructors and `RouterErrorExt`

Constructors in `topcoat::router::error`:

| Constructor | Status |
|---|---|
| `bad_request("msg")` | 400 (the message is shown to the client) |
| `unauthorized()` | 401 |
| `forbidden()` | 403 |
| `not_found()` | 404 |
| `too_many_requests(secs)` | 429 |
| `service_unavailable(secs)` | 503 |
| `internal_server_error(e)` | 500 |
| `rewrite(path, body)` | re-dispatch to another path |

Return them as `Err(not_found().into())`, or use the `RouterErrorExt` methods on `Option`/`Result`: `.ok_or_not_found()?`, `.ok_or_unauthorized()?`, `.ok_or_forbidden()?`, `.ok_or_bad_request("..")?`, `.ok_or_redirect(url)?`. Verified 401, 404 and 307.

Any other error becomes 500 without leaking its message.

### `Result` and error types

- `topcoat::Result<T, E = topcoat::Error>`. **`T` is required; plain `-> Result` does not compile** (verified).
- `topcoat::Error` has `From<E: std::error::Error + Send + Sync + 'static>`. Verified: `?` works on libsql errors, on argon2 `password_hash` errors, and on `tokio::task::JoinError`.
- It also has `Error::msg("..")` and `.context("..")`.

### JSON

```rust
#[route(POST)]
pub(crate) async fn create(cx: &Cx, Json(input): Json<NewNote>) -> Result<(StatusCode, Json<Note>)> {
    ...; Ok((StatusCode::CREATED, Json(note)))
}
```

Verified: the 201 response has `content-type: application/json`. A route can also return `String`, `&'static str`, a `(StatusCode, body)` tuple, or `SeeOther`.

---

## 6. App context and `#[memoize]` (VERIFIED)

```rust
module_router!()
    .app_context(Db { conn })          // any Send + Sync + 'static, keyed by exact type; duplicates panic
    .app_context(Config { .. })
    .build();

pub fn db(cx: &Cx) -> &Db { app_context::<Db>(cx) }      // panics if the type was not registered
// try_app_context::<T>(cx) -> Option<&T>
```

Verified: `libsql::Connection` stored in app context, inside a struct, works (`rustnote-deps-probe`).

```rust
#[memoize(as_ref)]                      // caches the owned Option<User> per request; callers get Option<&User>
pub async fn current_user(cx: &Cx) -> Option<User> {
    let hash = session::token_hash(cx).await.ok().flatten()?;
    db(cx).session_user(&hash)
}
pub async fn require_user(cx: &Cx) -> Result<&User> {
    Ok(current_user(cx).await.ok_or_redirect(href!(login::login_page).resolve(cx))?)
}
```

- The official pattern is "functions, not middlewares": call the guard inside each handler.
- `#[memoize]` without `as_ref` returns `&T`.
- Request-scoped values: `cx.with(value)`, `request_context::<T>(cx)`, `try_request_context` (UNVERIFIED).

---

## 7. Cookies, sessions and CSRF (VERIFIED: ran + curl)

### Router setup

`.cookies().sessions(SessionConfig::default())`. Import `cookie::RouterBuilderCookieExt` and `session::RouterBuilderSessionExt`.

### Session API (`topcoat::session`)

The framework only issues the token. **You store the records** (for example in a libSQL table).

| Function | Behaviour |
|---|---|
| `start(cx).await? -> Session { token_hash: TokenHash, expires_at: SystemTime }` | Always mints a fresh token |
| `token_hash(cx).await? -> Option<TokenHash>` | Hash of the token on the current request |
| `stop(cx).await? -> Option<TokenHash>` | Clears the cookie and returns the hash so you can delete the record |
| `refresh(cx)`, `rotate(cx) -> Rotation { revoked, session }` | UNVERIFIED |

- `TokenHash` is a 32-byte SHA-256. It `Deref`s to `[u8; 32]`, can be rebuilt with `TokenHash::new([u8; 32])`, and is `Hash + Eq`. Store it as a BLOB.
- The default cookie, as observed: `set-cookie: __Host-session=<base64>; HttpOnly; SameSite=Lax; Secure; Path=/; Max-Age=2592000` (lifetime 30 days).
- Logout sends `__Host-session=; ...; Max-Age=0`.
- Builder options: `SessionConfig::builder().token_store(CookieTokenStore::new().name("id")).lifetime(Duration).build()`.
- **`Secure` cannot be turned off** on `CookieTokenStore` (it is hard-coded). Only a custom `TokenStore` can change that.
- Local `http://127.0.0.1` and `http://localhost` still work:
  - curl 8.18 with `-c jar -b jar` sent the cookie back (verified).
  - Chrome treats localhost as a secure context (UNVERIFIED in a browser).
- There is no built-in in-memory or DB session store and no "user" model. Use `TokenStore` only to change the transport (for example a Bearer header).

### Cookie jar (`examples/cookies.rs`)

All of these were verified:

- `cookies(cx).get/add/remove` (needs the `Cookies` trait).
- `cookie!{ "theme" = v; Path = "/"; HttpOnly; SameSite = Lax }`
- `private_cookies(cx)` and `signed_cookies(cx)` (need `.app_context(Key::generate())`).
- `cookie_store::<T, _>(jar, "cart").parse_or_default().update(..).commit()?`

Cookie writes must happen before the response starts streaming.

### CSRF: built-in `OriginPolicy`

It is enabled by default, has no tokens, and runs before any layer.

- Non-GET/HEAD/OPTIONS requests are rejected with **403** when:
  - `Sec-Fetch-Site` is present and is not `same-origin` or `none` (verified 403 for `cross-site`), or
  - `Sec-Fetch-Site` is absent and `Origin` does not match `Host` (verified 403).
- Requests with neither header (for example curl) pass.
- Configure with `.origin_policy(OriginPolicy::new().trust_origins([..]).exempt_paths([..]))` or `OriginPolicy::dangerous_disable()`.
- Do not change state with GET.

---

## 8. Assets and Tailwind (VERIFIED)

### `asset!`

- `const STYLE: Asset = asset!("./style.css");`. A `./` path is relative to the source file.
- Render it with `<link rel="stylesheet" href=(STYLE)>`. The URL becomes `/_topcoat/assets/style-<hash>.css` with `cache-control: public, max-age=31536000, immutable` (verified).

### Bundle location

- The bundle goes **next to the executable**: `target/debug/assets/` or `target/release/assets/` (verified both).
- It contains `manifest.toml` plus the hashed files.
- Remote assets are cached in `target/topcoat/cache/assets`.

### Steps

1. `topcoat asset bundle [--release] [--bin X]` (or `topcoat dev`).
2. Load it with `.assets(AssetBundle::load().unwrap())` (`RouterBuilderAssetExt`).
   - **`AssetBundle::load()` returns `Err(NotFound "no asset bundle at ...")` after a plain `cargo run` without bundling** (verified panic).
   - For a custom `--out` directory, use `AssetBundle::load_dir("dist/assets")`.
   - Deploy the binary together with its matching `assets/` directory.

### Tailwind on Windows ARM64 (VERIFIED: fails by default; workaround works)

Setup: enable `features = ["tailwind"]` in `[dependencies]`, add the same as a `[build-dependencies]` with `default-features = false`, call `topcoat::tailwind::BuildConfig::new().render().unwrap()` in `build.rs`, and use `href=(tailwind::stylesheet!())` in the page.

Problems on this machine:

1. **The default build fails with HTTP 404.** topcoat asks for `tailwindcss-windows-arm64.exe` (pinned v4.3.2), but Tailwind publishes only `tailwindcss-windows-x64.exe`.
   - Workaround: download the x64 exe (it runs under emulation) and use `BuildConfig::new().executable_env("TAILWIND_CLI")` with `TAILWIND_CLI=<path>` (see `tailwind-probe/build.rs`).
2. The CLI is run with `TEMP=OUT_DIR/tailwind-scratch-*`. In a very deep project path this exceeds MAX_PATH and fails with `LoadLibrary failed: ファイル名または拡張子が長すぎます`.
   - Reproduced with TEMP at 229 characters; works at 151.
   - Keep the project path short (for example `C:\dev\rustnote`).
3. A target directory that is not git-ignored is scanned by Tailwind and crashed it (exit 0xC0000005). Keep `/target` in `.gitignore`.

With the workaround, the generated CSS contained `.text-blue-600{...}` (verified).

---

## 9. Runtime reactivity (`src/bin/runtime_demo/main.rs`)

VERIFIED: compiles, and the server-rendered HTML was checked with curl. **Browser behaviour was NOT tested** (UNVERIFIED).

Setup:

```rust
module_router!().discover().assets(AssetBundle::load()?).runtime().build()   // RouterBuilderRuntimeExt
```

- In `<head>`, include `topcoat::runtime::script()`. The script is itself an asset, so it needs the bundle.
- `.discover()` registers `#[procedure]` and `#[shard]` endpoints (verified paths `/_topcoat/runtime/procedures/<hash>` and `/_topcoat/runtime/shards/<hash>`).

```rust
let count = signal(cx, || 0usize);
<button @click=$(|_e| count.set(count.get() + 1))>"+1"</button>
<p>"Count: " $(count.get())</p>
<p :hidden=$(!open.get())>"..."</p>
<input :value=$(name.get()) @input=$(|e: Event| name.set(e.target.value))>

#[procedure] async fn double(value: usize) -> Result<usize> { Ok(value.saturating_mul(2)) }
<button @click=$(async |_e| { let d = double(count.get()).await; count.set(d); })>"x2"</button>

#[shard] async fn greeting(name: String) -> Result<impl View> { ... }
greeting(name: $(name.get()))
```

- The docs call the runtime "highly experimental".
- Procedures and shards are public HTTP endpoints, so validate and authorize inside them.
- Unsuffixed integers are `usize`.

---

## 10. Testing without a socket (VERIFIED: `cargo test`, 4 tests pass)

`Router::handle(Request) -> Response` needs no TCP. Put the router in a library crate (`src/lib.rs` with `pub mod app;`) so `tests/*.rs` can call `rustnote::app::router()`. Verified: `module_router!` discovery works in integration tests on Windows MSVC.

```rust
use topcoat::router::{Body, Method, StatusCode, header, request::Request, response::Response, to_bytes};
let req = Request::builder().method(Method::POST).uri("/login")
    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
    .body(Body::from("name=alice&password=secret")).unwrap();
let res = router.handle(req).await;
assert_eq!(res.status(), StatusCode::SEE_OTHER);
let cookie = res.headers()[header::SET_COOKIE].to_str().unwrap().split(';').next().unwrap().to_owned();
let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
```

See `topcoat-probe/tests/router.rs`. No reqwest is needed. The end-to-end curl script is `topcoat-probe/scripts/smoke.sh`.

---

## 11. Roadmap / not available in 0.10.0 (from the README roadmap, plus `main` after the release)

- **Not in 0.10.0:**
  - `topcoat new` (only on `main`).
  - Built-in authentication or login helpers.
  - Validations.
  - Rate limiting and other "easier middlewares".
  - Background jobs.
  - Static export and pre-rendering.
  - Official deploy docs.
  - i18n.
  - OpenAPI.
  - Image optimisation.
  - Markdown.
  - Islands.
  - More Toasty integration.
- **Unreleased on `main` (not in 0.10.0):**
  - `?` on `Result<T, &topcoat::Error>` from `#[memoize(as_ref)]` (#514).
  - Popover UI, `element_id(cx)`.
  - Module discovery scoped to the router root (#518).
  - Dev server restart on asset change.
  - Gitignore fixes for fmt/dev.
  - Rust 1.99 MSRV.

---

## 12. libsql + argon2 on Windows ARM64 (VERIFIED: built, ran and curled `rustnote-deps-probe`)

- `topcoat 0.10.0` + `libsql 0.10.0` (default features) + `argon2 0.6.0` build natively in about 1 minute and run.
  - The page lists rows from a local `probe.db` created with `libsql::Builder::new_local("probe.db").build().await?`, then `db.connect()?`, `conn.execute(sql, libsql::params![..]).await?` and `conn.query(..)` with `rows.next().await?` and `row.get::<i64>(0)?`.
- **The default libsql features need clang on Windows ARM64.** `tls` pulls in rustls and `ring`, and ring forces `clang` for aarch64-windows.
  - VERIFIED: with LLVM removed from PATH the build fails with `failed to find tool "clang"`.
  - This machine has LLVM at `C:\Program Files\LLVM\bin` and VS 2022 Build Tools.
  - Readers on Windows ARM64 must install LLVM (for example `winget install LLVM.LLVM`).
  - Alternative for local-only use: `libsql = { version = "=0.10.0", default-features = false, features = ["core"] }` builds **without** clang (VERIFIED). It cannot reach Turso remote, though.
  - Linux and Docker are not affected (UNVERIFIED).
- No bindgen or libclang is needed: libsql-ffi ships prebuilt bindings.
- **argon2 0.6 API (VERIFIED)** — different from 0.5:
  - No `SaltString` or `OsRng` is needed.
  - The salt is generated internally (default `getrandom` feature).
  - The output looks like `$argon2id$v=19$m=19456,t=2,p=1$...`.

```rust
use argon2::{Argon2, password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash}};
let phc = Argon2::default().hash_password(b"pw")?.to_string();
let ok = Argon2::default().verify_password(b"pw", &PasswordHash::new(&phc)?).is_ok();
// in async handlers: tokio::task::spawn_blocking(move || hash_password(&pw)).await?
```

---

## 13. Differences from 0.5.0 (VERIFIED against 0.5.0 sources plus compile errors on 0.10)

| 0.5 form | 0.10 status and correct form |
|---|---|
| `-> Result` with `view! {..}` returned directly (in 0.5, `Result` defaulted to `Result<View>` and `view!` evaluated to a Result) | **Gone.** Write `-> Result<impl View>` and `Ok(view! {..})`. Plain `Result` gives `E0107 missing generics` |
| Layout `slot: Result` rendered as `(slot?)` | **Gone.** Use `slot: Slot<'_>` and `(slot)`. The old form gives a type mismatch |
| `#[path_param] struct PostId(Uuid);` on a tuple struct (it also turned the module into `{post_id}`) | **Gone** (`cannot find attribute path_param`). Use `module_param!(post_id: Uuid, error = ..)` in module routing, or `path_param!(post_id: Uuid)` with an explicit `{post_id}` |
| `#[path_param] struct Slug(str)` | Use `path_param!(slug)`, which yields `&str` |
| `path_param!` sets the module segment (0.6–0.9 behaviour) | **Changed in 0.10**: it no longer does. Use `module_param!`. Old code silently serves the literal `/…/id` |
| `query_params::<T>(cx)` + `#[query_params(error = ..)]` | **Still exists, unchanged** |
| `app_context::<T>(cx)`, `.app_context(v)` | **Still exists** (`topcoat::context::app_context`, `RouterBuilder::app_context`) |
| `.cookies().sessions(SessionConfig::default())` | **Still exists** |
| `AssetBundle::load()` / `load_dir` / `.assets(..)` | **Still exist** |
| Bundle at `<target>/assets` (for example `target/assets`) | **Changed (0.6):** now next to the exe, `target/debug/assets` or `target/release/assets` |
| `#[page]`, `#[route(GET)]`, `#[layout]`, `#[layer]`, `module_router!`, `segment!` | Exist. Layouts now take a `Slot`. `href!` (0.6), `not_found!()` (0.6), relative `"./x"` paths (0.8.1), trailing-slash redirects (0.8.1) and `module_param!` (0.10) were added |
| `Router::builder().discover()` | Still exists, **but `use topcoat::router::RouterBuilderDiscoverExt;` is required** (`E0599` otherwise) |
| Unmatched requests ran layouts and layers | Since 0.6 they do not (only pathless layers run). Use `not_found!()` for a branded 404 |
| `OriginPolicy` | Global, default-on since 0.6 |
| Signal syntax (custom in 0.5) | Since 0.8, `signal(cx, || init)` is a plain function |

---

## 14. DO NOT CLAIM (these do not exist in 0.10.0)

- `topcoat new`, `topcoat build`, `topcoat serve`, or `topcoat dev --port` (use the `PORT` env var).
- A `HOST`/`PORT` setter on the router builder (e.g. `.port(8080)`, `.bind(..)`). `start` reads environment variables only.
- `#[path_param]` attribute or `#[query]`. A `query::<T>(cx)` function (the real one is `query_params::<T>(cx)`). `Path<T>`/`Query<T>`/`State<T>` extractors in the axum style (only `Form`, `Json`, `RawForm`, `Bytes`, `Multipart` (feature) and custom `FromRequest` exist).
- `Redirect::to(..)` or `redirect_see_other`. Use `see_other(..)` (303). Note that `redirect(..)` is **307**, not 303.
- Built-in session storage (memory, DB or libSQL), `session.insert("user_id", ..)`/`session.get(..)` key-value sessions, a `current_user` provided by the framework, `#[login_required]`, auth middleware, or CSRF token helpers (protection is origin-based only).
- A way to disable `Secure` on the default session cookie.
- `cx.db()` or any built-in DB integration with libSQL (Toasty is a separate project).
- `Router::test_client()` or a `TestClient` type. Use `router.handle(request)`.
- Interpolating a function's `impl View` directly as `(v)` without `.boxed()`.
- `view!` returning a `Result`, or `-> Result` without a type parameter.
- A Tailwind binary download that works on Windows ARM64 out of the box.
- Running on rustc ≤ 1.97.

---

## 15. Probe projects (kept in place)

All paths are under `C:\Users\oltot\AppData\Local\Temp\claude\C--Users-oltot-Documents-git-projects-def\f344debc-e3c6-4c0d-93d7-e55dbf9acedd\scratchpad\ref\`.

**`topcoat-probe\`**: the main verified app. It has a lib + bin layout, `rust-toolchain.toml` pinning 1.99.0, and `rustfmt.toml`.

| Path | What it demonstrates |
|---|---|
| `src/lib.rs` | `pub mod app; pub mod components;` |
| `src/main.rs` | `start(app::router())` |
| `src/app.rs` | `module_router!` + cookies + sessions + app_context; `not_found!()`; `#[layer]` access log; root `#[layout]` with `error_boundary` 404; in-memory `Db`; `#[memoize(as_ref)] current_user`; `require_user` |
| `src/app/notes.rs` | GET list with `#[query_params]`; POST `Form` → 303 |
| `src/app/notes/new.rs` | The new-note form |
| `src/app/notes/note_id.rs` | `module_param!`; show page; `POST ./delete` |
| `src/app/notes/note_id/edit.rs` | `#[page]` + `#[page(POST)]` with 422 re-render, or `Err(see_other)` |
| `src/app/login.rs` | Login form component; `session::start` |
| `src/app/logout.rs` | `session::stop` |
| `src/app/api/health.rs`, `src/app/api/notes.rs` | JSON routes, 201, 401 |
| `src/components.rs` | `#[component]` with `#[default]` and `Child`, `class!` |
| `src/bin/runtime_demo/` | signal / `$()` / `@click` / `:hidden` / `#[procedure]` / `#[shard]` / `asset!` / `AssetBundle` / `.runtime()` |
| `examples/hello.rs` | Explicit paths, `.discover()`, `path_param!` with `{user_id}`, request helpers, HOST/PORT |
| `examples/view_syntax.rs` | Full `view!` tour; HTML checked |
| `examples/path_param_gotcha.rs` | `module_param!` vs `path_param!` in 0.10 |
| `examples/cookies.rs` | Jar, `cookie!`, private cookies + `Key`, `cookie_store` |
| `tests/router.rs` | `Router::handle` tests |
| `scripts/smoke.sh` | curl end-to-end run; all outputs as described above |

**`rustnote-deps-probe\`**: topcoat + libsql (local file) + argon2, with `?` conversions. **`libsql-core-noclang-probe\`**: libsql `core` only, builds without clang. **`tailwind-probe\`**: tailwind feature with the `TAILWIND_CLI` workaround. Also `cli\bin\topcoat.exe` (CLI 0.10.0 installed with `--root`), `tools\tailwindcss-windows-x64.exe`, `old-src\` (0.5.0, 0.9.0 and tailwind crate sources), and `web\repo-v0.10.0\` (repo snapshot at the tag, including `llms-v0.10.0.txt`).

---

## 16. Not investigated / unverified (work stopped)

- Browser-side runtime behaviour (clicks, procedure calls, shard re-render).
- `live!`/`suspense`, SSE, WebSocket, multipart, mail and `topcoat ui`.
- `refresh`/`rotate` sessions; `cx.with`/`request_context`; `BodyLimit`; `public_dir` (`fs` feature).
- `trusted_proxies`/`client_ip` behind the Fly.io proxy.
- Turso remote connections (`Builder::new_remote(url, token)`), embedded replicas.
- A Linux/Docker build and the Fly.io deploy itself (Docker was not started).
- docs.rs HTML pages and the tokio.rs announcement blog post (crate-embedded docs were used instead).
- Housekeeping: I accidentally created an **empty directory `C:\tw-short`** outside the scratch area. A safety check blocked me from removing it. Please delete it manually.

---

## 17. Facts learned while building `Rust/rustnote/` (2026-10-17, all VERIFIED unless marked)

The full app is at `C:\Users\oltot\Documents\git-projects\def\Rust\rustnote\`. All of these came from compiling it with `cargo +1.99.0` and testing it with `Router::handle` and curl.

### Custom errors always become 500

- **The router has no extension point for custom error types.** `error_into_response` downcasts only Topcoat's own types: `ForbiddenError`, `BadRequestError`, `ContentTooLargeError`, `InternalServerError`, `NotFoundError`, `MethodNotAllowedError`, `RedirectError`, `SeeOther`, `UnauthorizedError`, `ServiceUnavailableError` and `TooManyRequestsError`. Anything else becomes a bare `500 internal server error`.
- **There is no 409 / Conflict router error at all.** Render a view with `(StatusCode::CONFLICT)` instead.
- RustNote's pattern for its own `AppError` (thiserror):
  - `?` converts `AppError` into `topcoat::Error`, through the blanket `From<E: std::error::Error + Send + Sync>`.
  - The root `#[layout]` wraps `(slot)` in `error_boundary(fallback: |error| error_page(cx, error), (slot))`.
  - `error_page` does `error.downcast_ref::<AppError>()` and renders `(app_error.status_code())` plus a message.
  - It also handles `NotFoundError` (404) and `BadRequestError` (400).
  - Every other error goes back up with `return Err(error)`, so redirects still work.
  - This only covers **pages**, because `#[route]` handlers are not wrapped by layouts.

### Form POSTs as pages

- **`#[page(POST)] async fn x(cx: &Cx) -> Result<()>`** works for form POSTs that always redirect: `Err(see_other(url).into())`. Because it is a page, layouts and their `error_boundary` wrap it, so an `AppError::NotFound` becomes a branded 404.
- A layer sees these redirects as `Err(SeeOther)`, not as a `Response`. Use `error.downcast_ref::<SeeOther>()` to log them as 303.

### `view!` inside a plain function

- **`view!` in a plain (non-component) function needs `cx =>`.** This includes a helper called from an `error_boundary` fallback.
- Without it, compilation fails with `error[E0425]: cannot find value '__cx' in this scope`.
- Working form: `fn error_page(cx: &Cx, error: Error) -> Result<impl View + '_> { Ok(view! { cx => ... }) }`.

### One `view!` per handler

- In page handlers that can re-render a form in several ways, return **one** `view!` at the end. Compute `(status, form_state)` first, then pass it to a single component call.
- RustNote does this in `signup.rs`, `notes.rs` and `edit.rs`.
- Why (UNVERIFIED as a compile error): each `view!` expansion has its own type, so two different `Ok(view!{...})` returns cannot share one `impl View`.

### `module_param!` with a domain type

- `module_param!(pub id: NoteId, error = not_found)` works with an application newtype that implements `FromStr` and `Display`. Here `NoteId::from_str` rejects ≤0 and non-numbers.
- The generated type is `Id(NoteId)`. Read it with `*path_param::<Id>(cx)?` and build links with `href!(show, Id(note_id))`.
- `/notes/abc`, `/notes/0` and `/notes/-1` all give 404.

### `#[memoize(as_ref)]` on a `Result`

- On `async fn f(cx: &Cx) -> Result<Option<User>, AppError>` it returns `Result<&Option<User>, &AppError>`.
- In 0.10.0, `?` does not work on `&topcoat::Error`, and `AppError` is not `Clone`. RustNote therefore maps `Err(e)` to `AppError::internal(e.to_string())`.
- The `?` support on `Result<T, &topcoat::Error>` (#514) exists only on `main`, not in 0.10.0.

### Testing with `asset!` without running the bundler

`Manifest` and `ManifestEntry` have public fields, and `Asset::id()` is public. Build the catalog by hand:

```rust
let manifest = Manifest { version: 1, assets: vec![ManifestEntry {
    id: STYLESHEET.id(), file: "style.css".into(), hash: String::new(), content_type: "text/css".into() }] };
router_builder.assets(AssetConfig::hosted_at("/test-assets", manifest))
```

The stylesheet then renders as `/test-assets/style.css`. `AssetBundle::load()` is not used in tests.

### Flash messages through cookies

1. `cookies(cx).add(cookie!{"flash" = code; Path = "/"; HttpOnly; SameSite = Lax; MaxAge = Duration::minutes(5)})`, written in the POST page body before `Err(see_other)`. The `Set-Cookie` is present on the 303.
2. In the next GET page body, `jar.get("flash")` and `jar.remove(Cookie::build(("flash", "")).path("/").build())` emit `Max-Age=0`.

Do this in page bodies, not in the layout. The order between the layout body and the page body is not specified.

### libSQL

- libSQL 0.10 `INSERT/UPDATE ... RETURNING` works on local files.
- A UNIQUE violation surfaces as a `libsql::Error` whose `to_string()` contains `UNIQUE constraint failed`. Map it to `RepoError::UniqueViolation`.
- **One connection per operation** (`database.connect()` each call) plus `PRAGMA foreign_keys = ON; PRAGMA busy_timeout = 5000;` through `execute_batch` works fine locally.
- `libsql::Database` is not `Clone`; wrap it in `Arc`.

### Async traits

`async-trait` 0.1.92 makes `Arc<dyn NoteRepository + Send + Sync>` with async methods compile. This is what volume 4's `AppState` design needs.

### Tooling

- `topcoat dev` and `topcoat asset bundle` honour `CARGO_TARGET_DIR`. The bundle lands in `$CARGO_TARGET_DIR/debug/assets`.
- `topcoat dev` also worked with `RUSTUP_TOOLCHAIN=1.99.0`, and the `dev.js` script tag was injected.
- **`topcoat fmt` formatter glitch:** a long string literal with `.to_string()` inside a component argument (`error: Some("…long…".to_string())`) gets reformatted as `.to_string(\n\n)`. It still compiles, but it is ugly. Workaround: move the string to a `const` or a local variable first.

### Windows and curl

- `curl.exe` on Windows (Git Bash) receives non-ASCII command-line arguments in the ANSI code page (CP932), so `--data-urlencode "title=買い物"` sends mojibake.
- Percent-encode UTF-8 yourself for smoke tests: `python -c "import urllib.parse,sys;print(urllib.parse.quote(sys.argv[1], safe=''))"`.

### Not verified

- **Docker build and Fly.io deploy of RustNote:** Docker Desktop was not started. The Docker Hub tags `rust:1.99.0-bookworm` and `debian:bookworm-slim` were confirmed to exist through the Hub API.
- **Turso remote connection:** the code path is `Builder::new_remote` and there was no Turso account.
