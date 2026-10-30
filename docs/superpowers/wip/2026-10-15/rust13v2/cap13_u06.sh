#!/usr/bin/env bash
# 第13巻 UNIT 06 の出力を取る（ログイン・お知らせ・共有の状態）。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust13"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
CAP="bash $H/cap.sh"
O="$H/out"
cd "$H/rustnote" || exit 1
check() { diff -rq src "$REAL/src" > /dev/null && diff -rq migrations "$REAL/migrations" > /dev/null && diff -rq tests "$REAL/tests" > /dev/null && echo "src・migrations・tests は教材と同じ" || { echo "教材と違う"; exit 1; }; }
check
export CARGO_TARGET_DIR="$H/target"
export PYTHONIOENCODING=utf-8
cargo build -q --examples || exit 1
bundle() { topcoat asset bundle --release > /dev/null 2>&1 || { echo "release の束を作れない"; exit 1; }; }
bundle
EXE="$H/target/release/rustnote.exe"
SESS="$H/target/debug/examples/u06_sessions.exe"

# ---- 例6-?: Argon2id（release）
$CAP u06_auth cargo run -q --release --example u06_auth > /dev/null
# ---- flash のユニットテスト
$CAP u06_lib_tests cargo test --lib -- flash:: > /dev/null

# ---- エラー1〜3
for e in e06_borrow e06_static e06_rc; do
  cp "$H/errs_u06/$e.rs" examples/
  $CAP "u06_err_${e#e06_}" cargo run -q --example "$e" > /dev/null
  rm "examples/$e.rs"
done

# ---- 本物のサーバー（release）で、ログイン・お知らせ・セッションの流れ
RUN="$H/u06run"
B=http://127.0.0.1:3014
start() {
  rm -rf "$RUN"; mkdir -p "$RUN"
  ( cd "$RUN" && PORT=3014 DATABASE_URL=u06.db "$EXE" > "$RUN/server.log" 2>&1 & )
  for i in $(seq 1 40); do
    sleep 0.25
    curl.exe -s -o "$RUN/discard" "$B/health" && return 0
  done
  echo "起動しない"; cat "$RUN/server.log"; exit 1
}
stop() { taskkill //F //IM rustnote.exe > /dev/null 2>&1; sleep 1; }
req() { local name=$1; shift; curl.exe -s -i "$@" > "$RUN/$name.raw"; python "$H/hdr13.py" "$RUN/$name.raw" | tr -d '\r' > "$O/$name.txt"; }
ck() { python "$H/cookie13.py" "$RUN/$1.raw" | tr -d '\r'; }
sess() { "$SESS" "$RUN/u06.db" 2>&1 | tr -d '\r' > "$O/$1.txt"; }
SIGNUP="email=alice%40example.com&password=correct+horse+battery&password_confirm=correct+horse+battery"
LOGIN="email=alice%40example.com&password=correct+horse+battery"
timing() {
  local name=$1
  : > "$O/$name.txt"
  for c in "A 登録済みのメール・違うパスワード|email=alice%40example.com&password=wrong-password" \
           "B 登録されていないメール|email=nobody%40example.com&password=wrong-password" \
           "C 形式の違うメール|email=nobody&password=wrong-password"; do
    local label=${c%%|*} data=${c#*|} ts=""
    for i in 1 2 3 4 5 6 7; do
      ts="$ts $(curl.exe -s -o "$RUN/discard" -w "%{http_code} %{time_total}" -d "$data" "$B/login")"
    done
    python "$H/median13.py" "$label" $ts | tr -d '\r' >> "$O/$name.txt"
  done
}

start
req u06_notes_anon "$B/notes"
req u06_login_flash -H "Cookie: flash=login_required" "$B/login"
req u06_login_bad_flash -H "Cookie: flash=%3Cscript%3E" "$B/login"
req u06_signup -d "$SIGNUP" "$B/signup"
C1=$(ck u06_signup)
sess u06_sess1
req u06_notes_auth -H "Cookie: $C1" "$B/notes"
req u06_login_again -H "Cookie: $C1" -d "$LOGIN" "$B/login"
C2=$(ck u06_login_again)
sess u06_sess2
req u06_old_cookie -H "Cookie: $C1" "$B/notes"
req u06_logout -H "Cookie: $C2" -X POST "$B/logout"
sess u06_sess3
req u06_after_logout -H "Cookie: $C2" "$B/notes"
timing u06_timing
grep -v "^$" "$RUN/server.log" | head -20 | tr -d '\r' > "$O/u06_server_log.txt"
stop

# 差し替え: edit <ファイル> <古い> <新しい>（古いものがちょうど1回あること）
edit() {
  python - "$@" <<'PY'
import sys
p, old, new = sys.argv[1], sys.argv[2], sys.argv[3]
s = open(p, encoding="utf-8").read()
assert s.count(old) == 1, old
open(p, "w", encoding="utf-8", newline="").write(s.replace(old, new))
PY
}

# ---- 問1: Flash に種類を足し、from_code の all に足し忘れる
edit src/flash.rs '    NoteDeleted,
}' '    NoteDeleted,
    NoteArchived,
}'
edit src/flash.rs '            Flash::NoteDeleted => "note_deleted",' '            Flash::NoteDeleted => "note_deleted",
            Flash::NoteArchived => "note_archived",'
edit src/flash.rs '            Flash::NoteDeleted => "ノートを削除しました",' '            Flash::NoteDeleted => "ノートを削除しました",
            Flash::NoteArchived => "ノートをしまいました",'
$CAP u06_q1_old cargo test --lib -- flash:: > /dev/null
edit src/flash.rs '        assert_eq!(Flash::from_code("<script>"), None);
    }' '        assert_eq!(Flash::from_code("<script>"), None);
    }

    #[test]
    fn archived_round_trips() {
        assert_eq!(
            Flash::from_code(Flash::NoteArchived.code()),
            Some(Flash::NoteArchived)
        );
    }'
$CAP u06_q1_new cargo test --lib -- flash:: > /dev/null
cp "$REAL/src/flash.rs" src/flash.rs

# ---- 問2: ログインのとき、前のセッションの記録を消さない
edit src/auth.rs '    // 前のセッションのクッキーが残っていれば、その記録は先に消しておく
    if let Some(old_hash) = session::token_hash(cx).await.map_err(AppError::internal)? {
        state.sessions.delete(&old_hash).await?;
    }
' '    // （問2の実験: 前のセッションの記録を消さない）
'
bundle
start
req u06_q2_signup -d "$SIGNUP" "$B/signup"
C1=$(ck u06_q2_signup)
req u06_q2_login_again -H "Cookie: $C1" -d "$LOGIN" "$B/login"
sess u06_q2_sess
req u06_q2_old_cookie -H "Cookie: $C1" "$B/notes"
stop
cp "$REAL/src/auth.rs" src/auth.rs

# ---- 問3: 登録されていないメールでは、ダミーの照合をせずにすぐ返す
edit src/auth.rs '        None => {
            verify_password(password, &DUMMY_HASH).await?;
            Ok(None)
        }' '        None => Ok(None),'
bundle
start
req u06_q3_signup -d "$SIGNUP" "$B/signup"
timing u06_q3_timing
stop
cp "$REAL/src/auth.rs" src/auth.rs

bundle
check
