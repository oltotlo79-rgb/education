use std::env;
use std::process;

fn main() {
    // コマンドラインで渡された 1 つ目の値を取り出す（無ければ None）
    let name = env::args().nth(1);

    match name {
        Some(name) => {
            // 標準出力へ書く（ふつうの結果）
            println!("こんにちは、{name} さん");
        }
        None => {
            // 標準エラー出力へ書く（エラーの知らせ）
            eprintln!("エラー: 名前を 1 つ渡してください（例: cargo run -- Hanako）");
            // 終了コード 2 でプロセスを終える（0 以外は「失敗」の合図）
            process::exit(2);
        }
    }
}
