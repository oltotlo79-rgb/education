"""第6巻 UNIT 10 のエラーの例と、問題の解答を、u10_errors.rs・u10_session.rs から作る。"""
import sys


def must(s, a, b):
    if s.count(a) != 1:
        sys.exit(f"見つからない: {a[:60]}")
    return s.replace(a, b)


err = open("web_lab/src/bin/u10_errors.rs", encoding="utf-8").read()
ses = open("web_lab/src/bin/u10_session.rs", encoding="utf-8").read()
head_err = "//! 第6巻 UNIT 10: エラーを、自分の画面（状態コード付き）に変える。404・400・リダイレクト・500。\n//! 実行: cargo run --bin u10_errors\n"
head_ses = "//! 第6巻 UNIT 10 の例10-2: クッキーとセッションで、「誰が見ているか」を覚える土台。\n//! 実行: cargo run --bin u10_session\n"

# エラー1: .cookies() を書き忘れた
e1 = must(ses, head_ses, "//! .sessions(…) は書いたが、.cookies() を書き忘れた\n")
e1 = must(e1, "        .cookies()\n", "")
open("errs/e10_nocookies.rs", "w", encoding="utf-8", newline="\n").write(e1)

# エラー2: 状態を変える操作を GET にした（ログアウトのリンク）
e2 = must(ses, head_ses, "//! ログアウトを、GET のリンクで済ませてしまった\n")
e2 = must(e2, "    /// POST /logout: セッションを終えて、記録も消す\n    #[page(POST)]", "    /// GET /logout: リンクを押すだけでログアウトする（してはいけない例）\n    #[page]")
open("errs/e10_getlogout.rs", "w", encoding="utf-8", newline="\n").write(e2)

# エラー3: error_page で、知らないエラーまで全部 500 の画面にしてしまった
e3 = must(err, head_err, "//! error_page で、知らないエラーまで全部つかまえてしまった\n")
e3 = must(e3, "    } else {\n        return Err(error);\n    };", "    } else {\n        // 何でも 500 の画面にする（リダイレクトも、ここに来る）\n        (StatusCode::INTERNAL_SERVER_ERROR, \"時間をおいてもう一度お試しください\".to_string())\n    };")
open("errs/e10_catchall.rs", "w", encoding="utf-8", newline="\n").write(e3)

# 問1: /boom（入出力のエラー）を、自分の 500 の画面にし、窓に記録する
q1 = must(err, head_err, "//! 問1の解答: 入出力のエラー（std::io::Error）を、記録してから、自分の 500 の画面にする。\n//! 実行: cargo run --bin u10_q1\n")
q1 = must(q1, "    } else {\n        return Err(error);\n    };", "    } else if let Some(io) = error.downcast_ref::<std::io::Error>() {\n        // 中のこと（ファイルの名前など）は、画面に出さずに窓へ書く\n        eprintln!(\"[error] {io}\");\n        (\n            StatusCode::INTERNAL_SERVER_ERROR,\n            \"時間をおいてもう一度お試しください\".to_string(),\n        )\n    } else {\n        return Err(error);\n    };")
open("web_lab/src/bin/u10_q1.rs", "w", encoding="utf-8", newline="\n").write(q1)

# 問2: テーマのクッキーに署名を付ける
q2 = must(ses, head_ses, "//! 問2の解答: テーマのクッキーに署名を付け、書き換えられたら読まないようにする。\n//! 実行: cargo run --bin u10_q2\n")
q2 = must(q2, "    cookie::{Cookie, Cookies, RouterBuilderCookieExt, cookies},", "    cookie::{Cookie, Cookies, Key, RouterBuilderCookieExt, signed_cookies},")
q2 = must(q2, "    let theme = cookies(cx)\n        .get(\"theme\")", "    // 署名が合わなければ None（書き換えられたクッキーは読まない）\n    let theme = signed_cookies(cx)\n        .get(\"theme\")")
q2 = must(q2, "        let jar = cookies(cx);\n", "        let jar = signed_cookies(cx);\n")
q2 = must(q2, "        .app_context(Sessions(Mutex::new(HashMap::new())))\n", "        .app_context(Sessions(Mutex::new(HashMap::new())))\n        // 署名の鍵。起動するたびに作る（本物は、秘密として保存した鍵を読む）\n        .app_context(Key::generate())\n")
open("web_lab/src/bin/u10_q2.rs", "w", encoding="utf-8", newline="\n").write(q2)
print("ok")
