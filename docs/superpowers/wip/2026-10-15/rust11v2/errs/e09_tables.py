# 第11巻 UNIT 09: Argon2 を同時に計算する数に上限を付ける（src/auth.rs）。超えた分は順番を待つ（背圧）。
SLOTS = [
    (
        """// ---------------------------------------------------------------------------
// パスワード（Argon2id）
// ---------------------------------------------------------------------------
""",
        """// ---------------------------------------------------------------------------
// パスワード（Argon2id）
// ---------------------------------------------------------------------------

/// Argon2 を同時に計算する数の上限（CPU の数）。超えた分は、ここで順番を待つ。
/// 1回に 19 MiB 使うので、同時の数を決めると、使うメモリの上限も決まる
static ARGON2_SLOTS: LazyLock<tokio::sync::Semaphore> = LazyLock::new(|| {
    tokio::sync::Semaphore::new(std::thread::available_parallelism().map_or(4, |n| n.get()))
});
""",
    ),
    (
        """    let password = password.clone();
    // 重い計算なので、非同期の処理を止めないよう別スレッドで行う
    let joined = tokio::task::spawn_blocking(move || {""",
        """    let password = password.clone();
    let _slot = ARGON2_SLOTS.acquire().await.map_err(AppError::internal)?;
    // 重い計算なので、非同期の処理を止めないよう別スレッドで行う
    let joined = tokio::task::spawn_blocking(move || {""",
    ),
    (
        """    let phc = phc.to_string();
    let joined = tokio::task::spawn_blocking(move || {""",
        """    let phc = phc.to_string();
    let _slot = ARGON2_SLOTS.acquire().await.map_err(AppError::internal)?;
    let joined = tokio::task::spawn_blocking(move || {""",
    ),
]
