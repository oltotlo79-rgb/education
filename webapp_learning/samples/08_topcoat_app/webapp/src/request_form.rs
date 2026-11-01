//! 依頼の入力フォーム（新しい依頼と、直して出し直すの 2 つの画面で使う）
//!
//! ファイルを送るフォームは `enctype="multipart/form-data"` で送られてくるので、
//! `Form<T>` ではなく `Multipart` で 1 項目ずつ読む。

use time::{Date, macros::format_description};
use topcoat::{
    Result,
    asset::asset,
    router::content::multipart::Multipart,
    view::{View, component, view},
};

use crate::{
    drawing,
    models::{AttachmentRow, Employee, RequestDetail, fmt_date},
    upload::{self, UploadedFile},
    views::error_list,
    workflow::{MAX_APPROVERS, RequestInput},
};

/// フォームに入っている値（まだ確かめていない。間違いがあれば、このまま画面に戻す）
#[derive(Debug, Clone, Default)]
pub struct FormValues {
    pub title: String,
    pub detail: String,
    pub due_date: String,
    /// 回覧者の社員の id（選ばなかった欄は空の文字）
    pub approvers: Vec<String>,
    /// 図面の JSON（隠し項目）
    pub drawing: String,
    pub files: Vec<UploadedFile>,
}

impl FormValues {
    /// 直すときは、今の依頼の値を入れておく
    pub fn from_request(r: &RequestDetail, approver_ids: &[i64]) -> FormValues {
        FormValues {
            title: r.title.clone(),
            detail: r.detail.clone(),
            due_date: fmt_date(r.due_date),
            approvers: approver_ids.iter().map(|id| id.to_string()).collect(),
            drawing: r.drawing.as_ref().map(|d| serde_json::to_string(&d.0).unwrap_or_default()).unwrap_or_default(),
            files: Vec::new(),
        }
    }

    /// 送られてきたフォームを、1 項目ずつ読む
    pub async fn read(mut multipart: Multipart) -> Result<FormValues> {
        let mut v = FormValues::default();
        while let Some(field) = multipart.next_field().await? {
            let name = field.name().unwrap_or("").to_string();
            if name == "files" {
                let file_name = upload::clean_name(field.file_name().unwrap_or(""));
                let bytes = field.bytes().await?;
                // ファイルを選ばなかったときも、名前が空の項目が 1 つ届くので、飛ばす
                if !file_name.is_empty() || !bytes.is_empty() {
                    v.files.push(UploadedFile { name: file_name, bytes: bytes.to_vec() });
                }
                continue;
            }
            let text = field.text().await?;
            match name.as_str() {
                "title" => v.title = text,
                "detail" => v.detail = text,
                "due_date" => v.due_date = text,
                "approver" => v.approvers.push(text),
                "drawing" => v.drawing = text,
                _ => {} // 知らない項目は捨てる
            }
        }
        Ok(v)
    }

    /// 確かめる。よければ、状態を変える処理に渡す形にする。だめなら、間違いの文を全部返す
    pub fn validate(&self) -> std::result::Result<RequestInput, Vec<String>> {
        let mut errors = Vec::new();

        let title = self.title.trim().to_string();
        if title.is_empty() {
            errors.push("件名を入れてください。".to_string());
        } else if title.chars().count() > 100 {
            errors.push("件名は 100 文字までです。".to_string());
        } else if title.chars().any(char::is_control) {
            // 入力欄は 1 行だが、細工した送信なら改行を入れられる。メールの件名などに入れさせない（10 の巻）
            errors.push("件名に、改行などは入れられません。".to_string());
        }
        if self.detail.chars().count() > 2000 {
            errors.push("内容は 2000 文字までです。".to_string());
        }
        let due_date = Date::parse(self.due_date.trim(), format_description!("[year]-[month]-[day]")).ok();
        if due_date.is_none() {
            errors.push("希望の納期を、日付で入れてください。".to_string());
        }
        let mut approver_ids = Vec::new();
        for a in self.approvers.iter().map(|a| a.trim()).filter(|a| !a.is_empty()) {
            match a.parse::<i64>() {
                Ok(id) => approver_ids.push(id),
                Err(_) => errors.push("回覧者の選び方が違います。".to_string()),
            }
        }
        if approver_ids.is_empty() {
            errors.push("回覧者を 1 人以上選んでください。".to_string());
        }
        let drawing = drawing::parse(&self.drawing).unwrap_or_else(|e| {
            errors.push(e);
            None
        });
        errors.extend(upload::check_all(&self.files));

        match (errors.is_empty(), due_date) {
            (true, Some(due_date)) => Ok(RequestInput {
                title,
                detail: self.detail.trim().to_string(),
                due_date,
                approver_ids,
                drawing,
                files: self.files.clone(),
            }),
            _ => Err(errors),
        }
    }
}

/// 入力フォーム。action を書かないので、今開いている URL（同じ所）へ POST で送る
#[component]
pub async fn request_form(
    values: &FormValues,
    choices: &[Employee],
    errors: &[String],
    attachments: &[AttachmentRow],
    submit_label: &str,
) -> Result<impl View> {
    Ok(view! {
        error_list(errors: errors)
        <form method="post" enctype="multipart/form-data" class="form-grid">
            <label for="title" class="required">"件名"</label>
            <input id="title" name="title" required="" maxlength="100" value=(&values.title)>

            <label for="detail">"内容"</label>
            <textarea id="detail" name="detail" rows="5" maxlength="2000">(&values.detail)</textarea>

            <label for="due" class="required">"希望の納期"</label>
            <input id="due" name="due_date" type="date" required="" value=(&values.due_date)>

            <label for="approver1" class="required">"回覧者"</label>
            <div class="approvers">
                for i in 0..MAX_APPROVERS {
                    let current = values.approvers.get(i).map(String::as_str).unwrap_or("");
                    <select id=(format!("approver{}", i + 1)) name="approver" aria-label=(format!("回覧者{}", i + 1))>
                        <option value="">(if i == 0 { "（選ぶ）" } else { "（なし）" })</option>
                        for e in choices {
                            let id = e.id.to_string();
                            <option value=(&id) selected=((id == current).then_some(""))>
                                (&e.dept) " " (&e.name) " " (&e.job_title)
                            </option>
                        }
                    </select>
                }
            </div>
            <span class="hint">"1 人目から順に回ります（1〜3 人）。"</span>

            <label>"図面"</label>
            <div>
                <svg id="pad" class="pad" viewBox="0 0 297 210" role="img" aria-label="図面を描く所"></svg>
                <div class="actions left">
                    <button type="button" id="pad-undo" class="btn-secondary">"1 つ戻す"</button>
                    <button type="button" id="pad-clear" class="btn-secondary">"全部消す"</button>
                </div>
                <input type="hidden" id="drawing" name="drawing" value=(&values.drawing)>
            </div>
            <span class="hint">"ドラッグで四角を描きます（JavaScript が動くブラウザで）。"</span>

            <label for="files">"添付ファイル"</label>
            <div>
                <input id="files" name="files" type="file" multiple="" accept=".jpg,.jpeg,.png,.pdf">
                if !attachments.is_empty() {
                    <p>"付いているファイル: "
                        for a in attachments {
                            <span>(&a.original_name) " "</span>
                        }
                    </p>
                }
            </div>
            <span class="hint">"JPG・PNG・PDF。1 つ 10MB まで・1 回に 5 つまで。"</span>

            <span></span>
            <div class="actions">
                <button type="submit" class="btn-primary">(submit_label)</button>
            </div>
        </form>
        <script src=(asset!("drawing.js"))></script>
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn values(title: &str) -> FormValues {
        FormValues { title: title.into(), due_date: "2026-11-30".into(), approvers: vec!["2".into()], ..FormValues::default() }
    }

    #[test]
    fn 正しい入力は通る() {
        let input = values("  治具の製作  ").validate().unwrap();
        assert_eq!(input.title, "治具の製作"); // 前後の空白は取る
        assert_eq!(input.approver_ids, vec![2]);
    }

    #[test]
    fn 件名に改行は入れられない() {
        let errors = values("治具の製作\r\nBcc: evil@example.com").validate().unwrap_err();
        assert_eq!(errors, vec!["件名に、改行などは入れられません。".to_string()]);
    }
}
