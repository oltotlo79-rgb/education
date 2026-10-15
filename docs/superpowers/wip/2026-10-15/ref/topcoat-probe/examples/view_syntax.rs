//! `view!` syntax tour. Run: cargo run --example view_syntax ; curl localhost:3000/
//! The rendered HTML is checked in the reference document.

use topcoat::{
    Result,
    context::Cx,
    router::{HeaderValue, StatusCode, header, module_router, page},
    view::{Child, Unescaped, View, ViewExt, attributes, class, component, view},
};

#[tokio::main]
async fn main() {
    topcoat::start(module_router!().build()).await.unwrap();
}

enum Status {
    Draft,
    Published { title: &'static str },
}

struct Post {
    id: u64,
    title: &'static str,
}

#[component]
async fn panel(title: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <section class="panel">
            <h2>(title)</h2>
            (child)
        </section>
    })
}

#[component]
async fn badge(#[into] label: String, #[default("gray")] color: &str) -> Result<impl View> {
    Ok(view! { <span class=(class!("badge", color))>(label)</span> })
}

// A plain async fn (not a component) must pass cx explicitly with `cx =>`.
async fn helper(cx: &Cx) -> Result<impl View> {
    Ok(view! { cx => badge(label: "from helper") })
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let name = "<b>Ada</b>"; // escaped when interpolated
    let trusted = Unescaped::new_unchecked("<em>trusted html</em>"); // NOT escaped
    let logged_in = true;
    let maybe_title: Option<&str> = None;
    let is_disabled = false;
    let status = Status::Published { title: "Hello" };
    let draft = Status::Draft;
    let posts = vec![Post { id: 1, title: "A" }, Post { id: 2, title: "B" }];
    let attrs = attributes! { id="save" type="submit" data-x=(42) };
    // An opaque `impl View` from a plain fn must be boxed before interpolation.
    let helper_view = helper(cx).await?.boxed();
    // A local `view!` value can be interpolated directly.
    let sidebar = view! { <aside>"sidebar"</aside> };

    Ok(view! {
        <!DOCTYPE html>
        <html>
            <body>
                // text must be quoted; expressions go in parentheses
                <p>
                    "Hello, "
                    (name)
                    "!"
                </p>
                <p>(trusted)</p>

                // if / else
                if logged_in {
                    <a href="/account">"Account"</a>
                } else {
                    <a href="/login">"Sign in"</a>
                }

                // if let
                if let Some(t) = maybe_title {
                    <h2>(t)</h2>
                }

                // match (multi-node arm needs braces; arms end with commas)
                match status {
                    Status::Draft => <span>"Draft"</span>,
                    Status::Published { title } => {
                        <strong>(title)</strong>
                        <em>"published"</em>
                    }
                }
                match draft {
                    Status::Draft => <span>"draft2"</span>,
                    _ => "",
                }

                // for with key, let binding inside markup
                <ul>
                    #[key(post.id)]
                    for post in posts {
                        let label = format!("#{} {}", post.id, post.title);
                        <li data-id=(post.id)>(label)</li>
                    }
                </ul>

                // conditional attributes
                <a
                    href="/posts"
                    if logged_in {
                        aria-current="page"
                        class="active"
                    }
                >
                    "Posts"
                </a>
                // bool attr: false omits it; Option attr: None omits it
                <button disabled=(is_disabled) title=(maybe_title)>"Save"</button>
                <input required="">

                // attribute collection spread
                <button (attrs)>"Spread"</button>

                // class! with conditional entry
                <div
                    class=(class!("box", "box--on" if logged_in, "box--off" if !logged_in))
                ></div>

                // components: named args, then children (no commas between children)
                panel(
                    title: "Panel",
                    <p>"child 1"</p>
                    badge(label: "new", color: "green")
                )
                badge(label: String::from("owned"))
                (helper_view)
                (sidebar)

                // response metadata from inside a view
                (StatusCode::IM_A_TEAPOT)
                ((header::CACHE_CONTROL, HeaderValue::from_static("no-store")))
            </body>
        </html>
    })
}
