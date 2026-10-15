// 設定値の「既定値」と「上限」は定数にしておく
const DEFAULT_PORT: u16 = 8080;
const MAX_RETRIES: u32 = 3;

fn main() {
    // 環境変数などから届いた「文字列の」ポート番号を想定する
    let port = " 3000 ";
    // 段階1: 空白を取り除く（文字列のまま）
    let port = port.trim();
    // 段階2: 数（u16）へ変換する。変換に失敗したら既定値を使う
    let port: u16 = port.parse().unwrap_or(DEFAULT_PORT);
    println!("待ち受けポート: {port}");

    // 数字ではない文字列だった場合も試す
    let broken = "abc";
    let broken: u16 = broken.trim().parse().unwrap_or(DEFAULT_PORT);
    println!("壊れた設定のときのポート: {broken}");

    // 変化する状態（試行回数）だけを mut にする
    let mut attempts = 0;
    while attempts < MAX_RETRIES {
        attempts += 1;
        println!("{attempts} 回目の接続を試みます");
    }
    println!("上限の {MAX_RETRIES} 回に達しました");
}
