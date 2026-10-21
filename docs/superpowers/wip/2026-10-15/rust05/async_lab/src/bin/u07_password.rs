use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

// パスワードを Argon2id でハッシュ化する（重い計算は spawn_blocking で別スレッドへ）
async fn hash_password(password: &str) -> Result<String, String> {
    // 別スレッドへ渡すので、借り物（&str）ではなく持ち主のいる String にする
    let password = password.to_string();
    let joined = tokio::task::spawn_blocking(move || {
        Argon2::default()
            .hash_password(password.as_bytes())
            .map(|hash| hash.to_string())
    })
    .await;
    // 外側の Result は「スレッドが panic しなかったか」、内側は「ハッシュ化に成功したか」
    match joined {
        Ok(Ok(phc)) => Ok(phc),
        Ok(Err(error)) => Err(format!("ハッシュ化に失敗: {error}")),
        Err(error) => Err(format!("計算のスレッドが異常終了: {error}")),
    }
}

// 入力されたパスワードが、保存してあるハッシュと一致するか調べる
async fn verify_password(password: &str, phc: &str) -> Result<bool, String> {
    let password = password.to_string();
    let phc = phc.to_string();
    let joined = tokio::task::spawn_blocking(move || {
        let parsed = PasswordHash::new(&phc)?;
        Ok::<bool, argon2::password_hash::Error>(
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed)
                .is_ok(),
        )
    })
    .await;
    match joined {
        Ok(Ok(matches)) => Ok(matches),
        Ok(Err(error)) => Err(format!("保存されたハッシュが読めない: {error}")),
        Err(error) => Err(format!("計算のスレッドが異常終了: {error}")),
    }
}

#[tokio::main]
async fn main() {
    let phc = hash_password("rust-is-fun").await.unwrap();
    println!("保存する文字列: {}...", &phc[..30]);
    println!(
        "正しいパスワード: {:?}",
        verify_password("rust-is-fun", &phc).await
    );
    println!(
        "違うパスワード:   {:?}",
        verify_password("rust-is-fum", &phc).await
    );
    println!(
        "壊れたハッシュ:   {:?}",
        verify_password("rust-is-fun", "abc").await
    );
}
