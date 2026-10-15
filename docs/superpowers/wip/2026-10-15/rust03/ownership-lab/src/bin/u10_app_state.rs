use std::sync::{Arc, Mutex, RwLock};
use std::thread;

// アプリ全体の設定（起動後は読むだけ）
struct Config {
    app_name: String,
    page_size: usize,
}

// すべてのハンドラで共有する状態
struct AppState {
    config: Config,
    // 書き換える値は、それぞれ鍵付きの箱に入れる
    request_count: Mutex<u64>,
    notes: RwLock<Vec<String>>,
}

// ノートを作るハンドラ（のつもり）
fn handle_create(state: &AppState, title: &str) {
    {
        let mut count = state.request_count.lock().unwrap();
        *count += 1;
    }
    // 書き込み用の鍵（write）を取り、追加したらすぐ返す
    state.notes.write().unwrap().push(title.to_string());
}

// ノート一覧を見るハンドラ（のつもり）
fn handle_list(state: &AppState) -> usize {
    *state.request_count.lock().unwrap() += 1;
    // 読み取り用の鍵（read）は、複数のスレッドが同時に持てる
    let notes = state.notes.read().unwrap();
    notes.len().min(state.config.page_size)
}

fn main() {
    let state = Arc::new(AppState {
        config: Config {
            app_name: String::from("RustNote"),
            page_size: 20,
        },
        request_count: Mutex::new(0),
        notes: RwLock::new(Vec::new()),
    });

    let mut handles = Vec::new();
    for i in 1..=3 {
        // リクエストごとに Arc の持ち手を1つ渡す（中身は複製しない）
        let state = Arc::clone(&state);
        handles.push(thread::spawn(move || {
            handle_create(&state, &format!("ノート{i}"));
            handle_list(&state)
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("[{}]", state.config.app_name);
    println!("リクエスト数: {}", state.request_count.lock().unwrap());
    println!("ノート件数: {}", state.notes.read().unwrap().len());
}
