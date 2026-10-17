// SQL ファイルを libSQL で1文ずつ実行し、結果を表の形で表示する道具。
// 使い方: cargo run -q --bin sql -- [--db ファイル] [--setup 準備用.sql]... 実行する.sql...
//   --db を付けないと、メモリの中だけの DB（終わると消える）を使う
//   --setup のファイルは、結果を表示せずに先に実行する（表やデータの準備用）
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::ExitCode;

use libsql::{Builder, Connection, Value};

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), String> {
    // コマンドラインの引数を読む
    let mut db_path = String::from(":memory:");
    let mut setup_files = Vec::new();
    let mut files = Vec::new();
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--db" => db_path = args.next().ok_or("--db の後にファイル名が必要です")?,
            "--setup" => setup_files.push(args.next().ok_or("--setup の後にファイル名が必要です")?),
            _ => files.push(arg),
        }
    }
    if files.is_empty() {
        return Err("使い方: sql [--db ファイル] [--setup 準備.sql]... 実行する.sql...".into());
    }

    // DB を開いて、接続を1本だけ作る（全部の文をこの接続で実行する）
    let db = Builder::new_local(&db_path)
        .build()
        .await
        .map_err(|e| e.to_string())?;
    let conn = db.connect().map_err(|e| e.to_string())?;

    for path in &setup_files {
        let sql = fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
        let statements = split_statements(&sql);
        for statement in &statements {
            run_statement(&conn, statement, false).await?;
        }
        println!("（準備: {path} の {} 文を実行しました）", statements.len());
    }
    for path in &files {
        let sql = fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
        for statement in split_statements(&sql) {
            // 文の1行目だけを表示する（2行目以降があれば … を付ける）
            let mut lines = statement.lines();
            let first = lines.next().unwrap_or("");
            let more = if lines.next().is_some() { " …" } else { "" };
            println!("sql> {first}{more}");
            // エラーが起きても止めずに、次の文へ進む
            if let Err(message) = run_statement(&conn, &statement, true).await {
                println!("エラー: {message}");
            }
            println!();
        }
    }
    Ok(())
}

// 1つの文を実行する。show が true なら結果を表示する
async fn run_statement(conn: &Connection, sql: &str, show: bool) -> Result<(), String> {
    let stmt = conn.prepare(sql).await.map_err(|e| e.to_string())?;
    if stmt.columns().is_empty() {
        // 行を返さない文（CREATE / INSERT / UPDATE / DELETE など）
        let changed = stmt.execute(()).await.map_err(|e| e.to_string())?;
        if show {
            let word = sql.split_whitespace().next().unwrap_or("").to_uppercase();
            if ["INSERT", "UPDATE", "DELETE", "REPLACE"].contains(&word.as_str()) {
                println!("OK（変更: {changed} 行）");
            } else {
                println!("OK");
            }
        }
        return Ok(());
    }

    // 行を返す文（SELECT など）: 列名と全部の行を読み取る
    let mut rows = stmt.query(()).await.map_err(|e| e.to_string())?;
    let count = rows.column_count();
    let names: Vec<String> = (0..count)
        .map(|i| rows.column_name(i).unwrap_or("?").to_string())
        .collect();
    let mut table: Vec<Vec<String>> = Vec::new();
    while let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
        let mut cells = Vec::new();
        for i in 0..count {
            let value = row.get_value(i).map_err(|e| e.to_string())?;
            cells.push(format_value(&value));
        }
        table.push(cells);
    }
    if !show {
        return Ok(());
    }
    if sql.to_uppercase().starts_with("EXPLAIN QUERY PLAN") {
        print_query_plan(&table);
    } else {
        print_table(&names, &table);
    }
    Ok(())
}

// 値を表示用の文字列にする
fn format_value(value: &Value) -> String {
    match value {
        Value::Null => "NULL".to_string(),
        Value::Integer(n) => n.to_string(),
        Value::Real(x) => x.to_string(),
        Value::Text(s) => s.clone(),
        Value::Blob(bytes) => {
            let hex: String = bytes.iter().map(|b| format!("{b:02X}")).collect();
            format!("x'{hex}'")
        }
    }
}

// 表の形で表示する
fn print_table(names: &[String], table: &[Vec<String>]) {
    // 列ごとに、いちばん長い値の幅を求める
    let mut widths: Vec<usize> = names.iter().map(|n| width(n)).collect();
    for cells in table {
        for (i, cell) in cells.iter().enumerate() {
            widths[i] = widths[i].max(width(cell));
        }
    }
    let line: Vec<String> = widths.iter().map(|w| "-".repeat(w + 2)).collect();
    let line = format!("+{}+", line.join("+"));
    println!("{line}");
    print_cells(names, &widths);
    println!("{line}");
    for cells in table {
        print_cells(cells, &widths);
    }
    if !table.is_empty() {
        println!("{line}");
    }
    println!("({} 行)", table.len());
}

fn print_cells(cells: &[String], widths: &[usize]) {
    let padded: Vec<String> = cells
        .iter()
        .zip(widths)
        .map(|(cell, w)| format!(" {cell}{} ", " ".repeat(w - width(cell))))
        .collect();
    println!("|{}|", padded.join("|"));
}

// EXPLAIN QUERY PLAN の結果（id, parent, notused, detail）を木の形で表示する
fn print_query_plan(table: &[Vec<String>]) {
    println!("QUERY PLAN");
    let mut depth: HashMap<String, usize> = HashMap::new();
    for cells in table {
        let d = depth.get(&cells[1]).map_or(0, |p| p + 1);
        depth.insert(cells[0].clone(), d);
        println!("{}- {}", "  ".repeat(d), cells[3]);
    }
}

// 画面での幅（日本語などの全角文字は 2、それ以外は 1）
fn width(s: &str) -> usize {
    s.chars()
        .map(|c| match c as u32 {
            0x1100..=0x115F | 0x2E80..=0xA4CF | 0xAC00..=0xD7A3 => 2,
            0xF900..=0xFAFF | 0xFE30..=0xFE4F | 0xFF00..=0xFF60 | 0xFFE0..=0xFFE6 => 2,
            _ => 1,
        })
        .sum()
}

// SQL を ; ごとの文に分ける（'…' や "…" の中の ; と、-- のコメントは区切りにしない）
fn split_statements(sql: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut chars = sql.chars().peekable();
    while let Some(c) = chars.next() {
        match quote {
            Some(q) => {
                current.push(c);
                if c == q {
                    quote = None;
                }
            }
            None if c == '\'' || c == '"' => {
                quote = Some(c);
                current.push(c);
            }
            None if c == '-' && chars.peek() == Some(&'-') => {
                // 行の終わりまでコメントなので読み飛ばす
                for c in chars.by_ref() {
                    if c == '\n' {
                        current.push('\n');
                        break;
                    }
                }
            }
            None if c == ';' => {
                push_statement(&mut statements, &current);
                current.clear();
            }
            None => current.push(c),
        }
    }
    push_statement(&mut statements, &current);
    statements
}

// 空行を取り除いてから、空でなければ文の一覧に加える
fn push_statement(statements: &mut Vec<String>, text: &str) {
    let lines: Vec<&str> = text
        .lines()
        .map(str::trim_end)
        .filter(|l| !l.trim().is_empty())
        .collect();
    if !lines.is_empty() {
        statements.push(lines.join("\n").trim().to_string());
    }
}
