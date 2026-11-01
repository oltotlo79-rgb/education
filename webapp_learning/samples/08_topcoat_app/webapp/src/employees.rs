//! 社員の CSV を取り込む（06 の import_employees と同じ決まり）
//!
//! - 1 行目の見出しは「従業員番号,氏名,所属,役職,メールアドレス」
//! - 1 行でも間違いがあれば、1 件も取り込まない（全部か、何もしないか）
//! - 同じ従業員番号の社員がいれば、氏名などを書きかえる。いなければ作る（パスワードは未設定）
//! - CSV は Excel の「CSV UTF-8（コンマ区切り）」で保存した物（先頭の BOM は読み飛ばす）

use sqlx::PgPool;

pub const COLUMNS: [&str; 5] = ["従業員番号", "氏名", "所属", "役職", "メールアドレス"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmployeeRow {
    pub employee_no: String,
    pub name: String,
    pub dept: String,
    pub job_title: String,
    pub email: String,
}

/// CSV を読んで、確かめる。間違いがあれば、行の番号付きの文を全部返す
pub fn read_csv(bytes: &[u8]) -> Result<Vec<EmployeeRow>, Vec<String>> {
    let bytes = bytes.strip_prefix(b"\xef\xbb\xbf").unwrap_or(bytes); // BOM を読み飛ばす
    if std::str::from_utf8(bytes).is_err() {
        return Err(vec!["文字コードが UTF-8 ではありません。Excel で「CSV UTF-8」として保存し直してください。".into()]);
    }
    let mut reader = csv::ReaderBuilder::new().has_headers(true).from_reader(bytes);
    let headers: Vec<String> = match reader.headers() {
        Ok(h) => h.iter().map(|s| s.trim().to_string()).collect(),
        Err(e) => return Err(vec![format!("1 行目を読めません: {e}")]),
    };
    if headers != COLUMNS {
        return Err(vec![format!("1 行目の見出しは {} にしてください（今: {}）", COLUMNS.join(","), headers.join(","))]);
    }

    let mut rows = Vec::new();
    let mut errors = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for (i, record) in reader.records().enumerate() {
        let n = i + 2; // 見出しが 1 行目なので、データは 2 行目から
        let record = match record {
            Ok(r) => r,
            Err(e) => {
                errors.push(format!("{n} 行目: 読めません（{e}）"));
                continue;
            }
        };
        let field = |k: usize| record.get(k).unwrap_or("").trim().to_string();
        let row = EmployeeRow { employee_no: field(0), name: field(1), dept: field(2), job_title: field(3), email: field(4) };

        if !(row.employee_no.len() == 6 && row.employee_no.bytes().all(|b| b.is_ascii_digit())) {
            errors.push(format!("{n} 行目: 従業員番号は 6 桁の数字にしてください（\"{}\"）", row.employee_no));
        } else if !seen.insert(row.employee_no.clone()) {
            errors.push(format!("{n} 行目: 従業員番号 {} が 2 回出てきます", row.employee_no));
        } else if row.name.is_empty() {
            errors.push(format!("{n} 行目: 氏名が空です"));
        } else if !looks_like_email(&row.email) {
            errors.push(format!("{n} 行目: メールアドレスの形が違います（\"{}\"）", row.email));
        } else {
            rows.push(row);
        }
    }
    if errors.is_empty() { Ok(rows) } else { Err(errors) }
}

/// メールアドレスの、おおまかな形（@ が 1 つ・前後が空でない・後ろに . がある）
fn looks_like_email(s: &str) -> bool {
    match s.split_once('@') {
        Some((local, domain)) => {
            !local.is_empty() && !domain.contains('@') && domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
        }
        None => false,
    }
}

/// 取り込む（1 つのトランザクションで）。新しく作った数と、書きかえた数を返す
pub async fn import(pool: &PgPool, rows: &[EmployeeRow]) -> sqlx::Result<(usize, usize)> {
    let mut tx = pool.begin().await?;
    let (mut created, mut updated) = (0, 0);
    for r in rows {
        // xmax = 0 なら新しく入った行（PostgreSQL の決まり。作ったか書きかえたかを見分ける）
        let inserted: bool = sqlx::query_scalar(
            "INSERT INTO employees (employee_no, name, dept, job_title, email) VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (employee_no) DO UPDATE
                SET name = EXCLUDED.name, dept = EXCLUDED.dept, job_title = EXCLUDED.job_title, email = EXCLUDED.email
             RETURNING (xmax = 0)",
        )
        .bind(&r.employee_no)
        .bind(&r.name)
        .bind(&r.dept)
        .bind(&r.job_title)
        .bind(&r.email)
        .fetch_one(&mut *tx)
        .await?;
        if inserted { created += 1 } else { updated += 1 }
    }
    tx.commit().await?;
    Ok((created, updated))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 正しいcsvを読める() {
        let rows = read_csv(include_bytes!("../sample_data/employees.csv")).unwrap();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[2].name, "山田 太郎");
    }

    #[test]
    fn 間違いのあるcsvは全部の間違いを返す() {
        let errors = read_csv(include_bytes!("../sample_data/employees_bad.csv")).unwrap_err();
        assert_eq!(
            errors,
            vec![
                "3 行目: 従業員番号は 6 桁の数字にしてください（\"30004\"）".to_string(),
                "4 行目: メールアドレスの形が違います（\"kato.example.com\"）".to_string(),
            ]
        );
    }

    #[test]
    fn shift_jisは断る() {
        // 「従」を Shift_JIS で書いた 2 バイト
        let errors = read_csv(b"\x8f\x5d,a\n").unwrap_err();
        assert!(errors[0].contains("UTF-8 ではありません"));
    }
}
