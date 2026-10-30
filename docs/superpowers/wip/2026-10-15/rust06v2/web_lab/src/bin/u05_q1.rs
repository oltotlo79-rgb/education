//! 問1の解答: note_card に、タグがあるときだけ <span class="tag"> を足したもの。
//! 実行: cargo run --bin u05_q1

use topcoat::{
    Result,
    router::{module_router, page},
    view::{Child, View, class, component, view},
};

/// 一覧に並べるノート（本物のアプリでは DB から読む）
struct Note {
    id: u32,
    title: &'static str,
    done: bool,
    tag: Option<&'static str>,
}

/// ノート1件を表示する部品
#[component]
async fn note_card(title: &str, done: bool, tag: Option<&str>) -> Result<impl View> {
    Ok(view! {
        <li class=(class!("note", "done" if done)) data-tag=(tag)>
            (title)
            if done {
                " ✓"
            }
            // タグがあるときだけ、中身を取り出して表示する
            if let Some(tag) = tag {
                <span class="tag">(tag)</span>
            }
        </li>
    })
}

/// 見出しと中身を枠で囲む部品（child に、呼んだ側の中身が入る）
#[component]
async fn panel(title: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <section class="panel">
            <h2>(title)</h2>
            (child)
        </section>
    })
}

/// GET /
#[page]
async fn home() -> Result<impl View> {
    let notes = vec![
        Note {
            id: 1,
            title: "牛乳を買う",
            done: true,
            tag: Some("買い物"),
        },
        Note {
            id: 2,
            title: "本を返す",
            done: false,
            tag: None,
        },
    ];
    let count = notes.len();
    let status = if notes.iter().all(|n| n.done) {
        "all"
    } else {
        "some"
    };

    Ok(view! {
        <h1>"ノート（" (count) " 件）"</h1>
        panel(
            title: "一覧",
            <ul>
                #[key(note.id)]
                for note in notes {
                    note_card(title: note.title, done: note.done, tag: note.tag)
                }
            </ul>
        )
        match status {
            "all" => <p>"すべて完了しました"</p>,
            _ => <p>"まだ残っています"</p>,
        }
        <input type="checkbox" checked=(count > 1) disabled=(false)>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
