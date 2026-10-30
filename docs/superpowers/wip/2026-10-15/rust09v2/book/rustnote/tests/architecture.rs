//! 依存の向きの決まり（どのモジュールが、どこを使ってよいか）を確かめるテスト。
//! src/ のファイルに書かれた `crate::モジュール` と `クレート名::` を調べる。
//! 実行: cargo test --test architecture

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

/// RustNote のモジュール（src/lib.rs の pub mod の並び）
const MODULES: [&str; 10] = [
    "auth",
    "config",
    "db",
    "domain",
    "error",
    "flash",
    "repository",
    "routes",
    "state",
    "views",
];

/// 調べる外のクレート（Cargo.toml の [dependencies]）
const CRATES: [&str; 8] = [
    "anyhow",
    "argon2",
    "async_trait",
    "dotenvy",
    "libsql",
    "serde",
    "thiserror",
    "topcoat",
];

/// モジュールのファイル（src/routes.rs と、src/routes/ の下のすべて）を集める
fn files_of(module: &str) -> Vec<PathBuf> {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut found = vec![src.join(format!("{module}.rs"))];
    let mut dirs = vec![src.join(module)];
    while let Some(dir) = dirs.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue; // 子モジュールのフォルダが無いモジュールもある
        };
        for entry in entries {
            let path = entry.expect("フォルダの中身を読めません").path();
            if path.is_dir() {
                dirs.push(path);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                found.push(path);
            }
        }
    }
    found
}

/// `crate::flash` のような道のりが、名前の一部としてではなく書かれているか
fn uses_module(line: &str, module: &str) -> bool {
    let path = format!("crate::{module}");
    line.match_indices(&path).any(|(at, _)| {
        let after = line[at + path.len()..].chars().next();
        !after.is_some_and(|c| c.is_alphanumeric() || c == '_')
    })
}

/// `libsql::` のような外のクレートの名前が、別の名前の一部としてではなく書かれているか
fn uses_crate(line: &str, name: &str) -> bool {
    let path = format!("{name}::");
    line.match_indices(&path).any(|(at, _)| {
        let before = line[..at].chars().next_back();
        !before.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == ':')
    })
}

/// モジュールが使っている（ほかのモジュール, 外のクレート）
fn uses_of(module: &str) -> (BTreeSet<&'static str>, BTreeSet<&'static str>) {
    let mut modules = BTreeSet::new();
    let mut crates = BTreeSet::new();
    for file in files_of(module) {
        let text = fs::read_to_string(&file).expect("ファイルを読めません");
        for line in text.lines() {
            if line.trim_start().starts_with("//") {
                continue; // コメントの行は数えない
            }
            for other in MODULES {
                if other != module && uses_module(line, other) {
                    modules.insert(other);
                }
            }
            for name in CRATES {
                if uses_crate(line, name) {
                    crates.insert(name);
                }
            }
        }
    }
    (modules, crates)
}

// R-1: domain（アプリの決まりの中心）は、ほかのモジュールも、Web と DB のクレートも使わない
#[test]
fn r1_domain_stands_alone() {
    let (modules, crates) = uses_of("domain");
    assert!(
        modules.is_empty(),
        "domain が使っているモジュール: {modules:?}"
    );
    assert_eq!(
        crates,
        BTreeSet::from(["thiserror"]),
        "domain が使ってよいのは thiserror だけです"
    );
}

// R-2: SQL を送る libsql を使ってよいのは、db・repository・error（DB のエラーの変換）だけ
#[test]
fn r2_only_the_db_side_uses_libsql() {
    for module in MODULES {
        if ["db", "repository", "error"].contains(&module) {
            continue;
        }
        let (_, crates) = uses_of(module);
        assert!(
            !crates.contains("libsql"),
            "{module} が libsql を使っています。SQL は repository に書きます"
        );
    }
}

// R-3: repository（データのしまい方）は、画面とログインの部品を知らない
#[test]
fn r3_repository_does_not_know_screens() {
    let (modules, _) = uses_of("repository");
    for screen in ["auth", "flash", "routes", "state", "views"] {
        assert!(
            !modules.contains(screen),
            "repository が {screen} を使っています"
        );
    }
}

// R-4: routes（URL ごとの処理）は db を直接使わず、AppState のリポジトリを通す
#[test]
fn r4_routes_go_through_repositories() {
    let (modules, _) = uses_of("routes");
    assert!(
        !modules.contains("db"),
        "routes が db を直接使っています。state のリポジトリを通します"
    );
}

// R-5: パスワードのハッシュ（argon2）を扱うのは auth だけ
#[test]
fn r5_only_auth_hashes_passwords() {
    for module in MODULES {
        if module == "auth" {
            continue;
        }
        let (_, crates) = uses_of(module);
        assert!(
            !crates.contains("argon2"),
            "{module} が argon2 を使っています"
        );
    }
}

// R-6（問2）: views（画面の部品）は、DB とリポジトリに触らず、渡された値を表示するだけ
#[test]
fn r6_views_only_show_given_values() {
    let (modules, crates) = uses_of("views");
    for storage in ["db", "repository", "state"] {
        assert!(
            !modules.contains(storage),
            "views が {storage} を使っています"
        );
    }
    assert!(!crates.contains("libsql"), "views が libsql を使っています");
}
