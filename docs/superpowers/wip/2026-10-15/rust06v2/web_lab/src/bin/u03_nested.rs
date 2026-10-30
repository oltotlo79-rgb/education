//! 第6巻 UNIT 03 の例3-2: レイアウトを入れ子にし、レイヤーで処理の時間を測る。
//! 実行: cargo run --bin u03_nested

use topcoat::{
    Result,
    context::Cx,
    router::{
        Body, Next, Slot, layer, layout, module_router, page,
        request::{method, uri},
        response::Response,
    },
    view::{View, view},
};

/// すべての要求を包むレイヤー: 処理にかかった時間を表示する
#[layer]
async fn timing(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let started = std::time::Instant::now();
    let result = next.run(cx, body).await;
    let status = match &result {
        Ok(response) => response.status().to_string(),
        Err(error) => format!("エラー（{error}）"),
    };
    println!(
        "{} {} -> {status}（{:?}）",
        method(cx),
        uri(cx).path(),
        started.elapsed()
    );
    result
}

/// 外側のレイアウト（/ より下の、すべてのページ）
#[layout]
async fn root_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <html>
            <body>
                <header>"サイトの見出し"</header>
                (slot)
            </body>
        </html>
    })
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! { <h1>"トップ"</h1> })
}

mod settings {
    use super::*;

    /// 内側のレイアウト（/settings より下のページだけ）
    #[layout]
    async fn settings_layout(slot: Slot<'_>) -> Result<impl View> {
        Ok(view! {
            <div class="settings">
                <nav>"設定のメニュー"</nav>
                (slot)
            </div>
        })
    }

    /// GET /settings
    #[page]
    async fn settings_top() -> Result<impl View> {
        Ok(view! { <h1>"設定"</h1> })
    }

    mod profile {
        use super::*;

        /// GET /settings/profile
        #[page]
        async fn profile() -> Result<impl View> {
            Ok(view! { <h1>"プロフィール"</h1> })
        }
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
