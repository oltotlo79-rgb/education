//! 第8巻 UNIT 09: 外の認証サービスから戻ってくる（コールバック）とき、state を確かめるかどうかで、
//! 何が変わるかを、判断の部分だけ取り出して試す（本物の認証サービスには送らない）。
//! 攻撃する人は、自分のアカウントでログインを始めて受け取った code と state を、
//! alice のブラウザに開かせる（ログインの CSRF）。
//!
//! 実行: cargo run -q --example callback

/// alice のブラウザのセッションに、ログインを始めたときに覚えておく値
struct Pending {
    state: String,
}

/// コールバックに来た値
struct Callback {
    code: &'static str,
    state: &'static str,
}

/// state を確かめる版
fn handle_checked(pending: Option<&Pending>, callback: &Callback) -> String {
    match pending {
        Some(pending) if pending.state == callback.state => {
            format!("code {} で、ログインを続ける", callback.code)
        }
        _ => "断る（このブラウザが始めたログインではない）".to_string(),
    }
}

/// state を確かめない版（わざと間違えた版）
fn handle_unchecked(_pending: Option<&Pending>, callback: &Callback) -> String {
    format!("code {} で、ログインを続ける", callback.code)
}

fn main() {
    let alice = Pending {
        state: "alice-no-state-PcI4617p".to_string(),
    };
    let own = Callback {
        code: "alice-no-code",
        state: "alice-no-state-PcI4617p",
    };
    let attack = Callback {
        code: "kougeki-suru-hito-no-code",
        state: "kougeki-suru-hito-no-state",
    };
    println!("--- state を確かめる版");
    println!(
        "alice 本人のコールバック → {}",
        handle_checked(Some(&alice), &own)
    );
    println!(
        "攻撃する人が開かせたコールバック → {}",
        handle_checked(Some(&alice), &attack)
    );
    println!(
        "ログインを始めていないブラウザ → {}",
        handle_checked(None, &attack)
    );
    println!("--- state を確かめない版");
    println!(
        "攻撃する人が開かせたコールバック → {}",
        handle_unchecked(Some(&alice), &attack)
    );
    println!(
        "ログインを始めていないブラウザ → {}",
        handle_unchecked(None, &attack)
    );
}
