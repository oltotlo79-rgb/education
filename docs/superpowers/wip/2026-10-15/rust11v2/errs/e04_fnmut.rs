//! 第11巻 UNIT 04 エラー2: TestRunner::run に渡す関数の中で、外の変数（seen）を書き換えようとした。
//! どんな入力でも 500 にならず、決めた状態コードだけが返ることを確かめる。
//!
//! 実行: cargo test --test e04_fnmut

mod common;

use std::collections::BTreeMap;

use common::test_app;
use proptest::prelude::*;
use proptest::test_runner::{Config, TestRunner};

/// バイトの並びを、全部 %XX の形にする（UTF-8 として正しくないバイトも送れる）
fn percent_all(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("%{byte:02X}")).collect()
}

/// ページ番号として送る文字列: 大きな数・負の数・数字でないもの
fn page_text() -> impl Strategy<Value = String> {
    prop_oneof![
        any::<u64>().prop_map(|n| n.to_string()),
        any::<i32>().prop_map(|n| n.to_string()),
        "[-+0-9a-z.]{0,8}",
    ]
}

#[test]
fn random_inputs_never_give_500() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let app = runtime.block_on(test_app());
    let alice = runtime.block_on(app.sign_up("alice@example.com", "correct horse battery"));
    let mut runner = TestRunner::new(Config::with_cases(300));
    let mut seen = BTreeMap::new();

    let strategy = (
        proptest::collection::vec(any::<u8>(), 0..40),
        page_text(),
        proptest::collection::vec(any::<u8>(), 0..40),
        proptest::collection::vec(any::<u8>(), 0..40),
    );
    let result = runner.run(&strategy, |(q, page, title, body)| {
        runtime.block_on(async {
            // 一覧の検索とページ送り
            let uri = format!("/notes?q={}&page={page}", percent_all(&q));
            let status = app.get(&uri, Some(&alice)).await.status();
            *seen.entry(format!("GET /notes → {status}")).or_insert(0) += 1;
            prop_assert!(
                status.as_u16() == 200 || status.as_u16() == 400,
                "{uri} → {status}"
            );

            // 作成のフォーム
            let form = format!("title={}&body={}", percent_all(&title), percent_all(&body));
            let status = app.post_form("/notes", &form, Some(&alice)).await.status();
            *seen.entry(format!("POST /notes → {status}")).or_insert(0) += 1;
            prop_assert!(
                status.as_u16() == 303 || status.as_u16() == 400,
                "{form} → {status}"
            );
            Ok(())
        })
    });
    for (what, count) in &seen {
        println!("{what}: {count} 回");
    }
    result.unwrap();
}
