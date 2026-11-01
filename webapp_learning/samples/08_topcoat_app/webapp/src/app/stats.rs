//! /stats … 集計（状態ごとの件数と、担当者ごとの完了の件数）

use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::{auth::require_user, models::Status, queries, state::state, views::status_badge};

#[page]
pub async fn stats_page(cx: &Cx) -> Result<impl View> {
    require_user(cx).await?;
    let s = queries::stats(&state(cx).pool).await?;
    // 状態の文字を enum にする（知らない値は飛ばす）
    let mut by_status: Vec<(Status, i64)> =
        s.by_status.into_iter().filter_map(|(k, n)| Status::try_from(k).ok().map(|st| (st, n))).collect();
    // 流れの順（回覧中 → … → 完了）に並べる
    by_status.sort_by_key(|(st, _)| Status::ALL.iter().position(|a| a == st));
    let max = by_status.iter().map(|(_, n)| *n).max().unwrap_or(1).max(1);
    Ok(view! {
        <h1>"集計"</h1>
        <h2>"状態ごとの件数"</h2>
        <table>
            <thead><tr><th>"状態"</th><th>"件数"</th><th>"棒グラフ"</th></tr></thead>
            <tbody>
                for (status, n) in &by_status {
                    <tr>
                        <td>status_badge(status: *status)</td>
                        <td>(n)</td>
                        <td><div class="bar" style=(format!("width: {}%", n * 100 / max))></div></td>
                    </tr>
                }
            </tbody>
        </table>
        <h2>"担当者ごとの完了の件数"</h2>
        if s.done_by_assignee.is_empty() {
            <p>"まだありません。"</p>
        } else {
            <table>
                <thead><tr><th>"担当者"</th><th>"完了"</th></tr></thead>
                <tbody>
                    for (name, n) in &s.done_by_assignee {
                        <tr><td>(name)</td><td>(n)</td></tr>
                    }
                </tbody>
            </table>
        }
    })
}
