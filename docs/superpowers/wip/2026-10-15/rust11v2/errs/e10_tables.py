# 第11巻 UNIT 10 で、RustNote のコピーに当てる書き換え。

# libsql の使わない機能（replication・sync）を外す（Cargo.toml）
FEATURES = [
    (
        'libsql = "=0.10.0"\n',
        'libsql = { version = "=0.10.0", default-features = false, features = ["core", "remote", "tls"] }\n',
    )
]

# エラー1: Cargo.toml に依存を足したのに、Cargo.lock を更新しないまま --locked で組み立てる
NEW_DEP = [
    ('topcoat = "=0.11.0"\n', 'topcoat = "=0.11.0"\n# 数字を速く文字にする（試しに足した）\nitoa = "1"\n')
]

# エラー2: 使わない変数を残した（src/routes/health.rs）
UNUSED = [
    (
        "    match app_state(cx).db.ping().await {\n",
        "    let started = std::time::Instant::now();\n    match app_state(cx).db.ping().await {\n",
    )
]

# エラー3: 字下げを崩した（src/routes/health.rs）
FORMAT = [
    (
        "        Ok(()) => Ok((StatusCode::OK, Json(Health { status: \"ok\" }))),\n",
        "        Ok(()) => Ok((StatusCode::OK, Json(Health { status: \"ok\" })))   ,\n",
    )
]
