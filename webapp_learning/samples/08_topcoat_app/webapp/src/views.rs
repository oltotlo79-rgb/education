//! いくつかの画面で使う、画面の部品（コンポーネント）
//!
//! `#[component]` を付けた関数は、`view!` の中で `status_badge(status: r.status)` のように呼べる。
//! `view!` の中の `(式)` は、HTML として危ない文字（`<` `&` など）を自動で置きかえる（XSS を防ぐ）。

use topcoat::{
    Result,
    view::{View, component, view},
};

use crate::{
    drawing::{Drawing, Shape},
    models::{RequestDetail, Status, fmt_date},
};

/// 状態の札（01 の CSS の .badge）
#[component]
pub async fn status_badge(status: Status) -> Result<impl View> {
    Ok(view! { <span class=(status.css_class())>(status.label())</span> })
}

/// 間違いの一覧（無ければ何も出さない）
#[component]
pub async fn error_list(errors: &[String]) -> Result<impl View> {
    Ok(view! {
        if !errors.is_empty() {
            <div class="errors" role="alert">
                <ul>
                    for e in errors {
                        <li>(e)</li>
                    }
                </ul>
            </div>
        }
    })
}

/// 数を、SVG の属性に書く文字にする（小数点以下が 0 なら整数で）
fn num(v: f64) -> String {
    if v.fract() == 0.0 { format!("{}", v as i64) } else { format!("{v:.1}") }
}

/// 図面を SVG で描く（サーバーで描くので、見るだけの画面には JavaScript が要らない）
#[component]
pub async fn drawing_svg(drawing: &Drawing) -> Result<impl View> {
    Ok(view! {
        <svg class="pad" viewBox=(format!("0 0 {} {}", num(drawing.width_mm), num(drawing.height_mm))) role="img" aria-label="図面">
            for shape in &drawing.shapes {
                match shape {
                    Shape::Rect { x, y, width, height } =>
                        <rect x=(num(*x)) y=(num(*y)) width=(num(*width)) height=(num(*height))></rect>,
                    Shape::Line { x1, y1, x2, y2 } =>
                        <line x1=(num(*x1)) y1=(num(*y1)) x2=(num(*x2)) y2=(num(*y2))></line>,
                    Shape::Text { x, y, text } =>
                        <text x=(num(*x)) y=(num(*y))>(text)</text>,
                }
            }
        </svg>
    })
}

/// 依頼の中身の表（詳しい画面と、承認の画面で使う）
#[component]
pub async fn request_table(r: &RequestDetail) -> Result<impl View> {
    Ok(view! {
        <table class="kv">
            <tr><th>"件名"</th><td>(&r.title)</td></tr>
            <tr><th>"依頼者"</th><td>(&r.requester_name) "（" (&r.requester_dept) "）"</td></tr>
            <tr><th>"希望の納期"</th><td>(fmt_date(r.due_date))</td></tr>
            <tr><th>"状態"</th><td>status_badge(status: r.status)</td></tr>
            if let Some(name) = &r.assignee_name {
                <tr><th>"担当"</th><td>(name)</td></tr>
            }
            <tr><th>"内容"</th><td class="pre">(&r.detail)</td></tr>
            if !r.return_comments.is_empty() {
                <tr><th>"差し戻しのコメント"</th><td class="pre">(&r.return_comments)</td></tr>
            }
        </table>
        if let Some(d) = &r.drawing {
            <h2>"図面"</h2>
            drawing_svg(drawing: &d.0)
        }
    })
}
