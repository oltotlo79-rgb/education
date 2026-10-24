//! RustNote の src/ を読み、モジュールどうしの依存（どこがどこを使っているか）を一覧にする。
//! 実行: cargo run -q --example u04_deps

use std::collections::{BTreeMap, BTreeSet};
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

/// 調べる外のクレート（Cargo.toml の [dependencies]。名前の - は _ にする）
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

/// フォルダの下の .rs ファイルを、すべて集める
fn rust_files(dir: &Path, found: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("フォルダを読めません") {
        let path = entry.expect("フォルダの中身を読めません").path();
        if path.is_dir() {
            rust_files(&path, found);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            found.push(path);
        }
    }
}

/// ファイルが、どのモジュールに入るかを決める（src/routes/notes.rs → routes）
fn module_of(src: &Path, file: &Path) -> String {
    let relative = file.strip_prefix(src).expect("src の下のファイルです");
    let first = relative.iter().next().expect("ファイル名があります");
    first.to_string_lossy().trim_end_matches(".rs").to_string()
}

/// `crate::flash` のような、ほかのモジュールへの道のりが書かれているか
/// （`crate::flashy` のような、名前の続きがあるものは別のもの）
fn uses_module(line: &str, module: &str) -> bool {
    let path = format!("crate::{module}");
    line.match_indices(&path).any(|(at, _)| {
        let after = line[at + path.len()..].chars().next();
        !after.is_some_and(|c| c.is_alphanumeric() || c == '_')
    })
}

/// `libsql::` のような、外のクレートの名前が書かれているか
/// （`my_libsql::` や `x::libsql::` のような、別の名前の一部は数えない）
fn uses_crate(line: &str, name: &str) -> bool {
    let path = format!("{name}::");
    line.match_indices(&path).any(|(at, _)| {
        let before = line[..at].chars().next_back();
        !before.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == ':')
    })
}

fn main() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    rust_files(&src, &mut files);

    // モジュールの名前 → （使っているモジュール, 使っている外のクレート）
    let mut uses: BTreeMap<String, (BTreeSet<&str>, BTreeSet<&str>)> = BTreeMap::new();
    for file in &files {
        let module = module_of(&src, file);
        // lib.rs（部品の一覧）と main.rs（入口）は、ここでは数えない
        if !MODULES.contains(&module.as_str()) {
            continue;
        }
        let text = fs::read_to_string(file).expect("ファイルを読めません");
        let (modules, crates) = uses.entry(module.clone()).or_default();
        for line in text.lines() {
            // コメントの行に書かれた名前は、使っているうちに入らない
            if line.trim_start().starts_with("//") {
                continue;
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

    println!(
        "--- モジュールごとの依存（{} 個のファイルを調べた）",
        files.len()
    );
    for (module, (modules, crates)) in &uses {
        let list = |names: &BTreeSet<&str>| {
            if names.is_empty() {
                "（なし）".to_string()
            } else {
                names.iter().copied().collect::<Vec<_>>().join(" ")
            }
        };
        println!("{module:<11}→ {}", list(modules));
        println!("{:<11}  外: {}", "", list(crates));
    }

    println!("--- 互いに使い合っている組");
    for (a, (a_uses, _)) in &uses {
        for b in a_uses {
            let b_uses_a = uses
                .get(*b)
                .is_some_and(|(b_uses, _)| b_uses.contains(a.as_str()));
            if b_uses_a && a.as_str() < *b {
                println!("{a} ⇄ {b}");
            }
        }
    }
}
