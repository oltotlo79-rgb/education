cd ~\rust-study
mkdir old
cd old
# 「このフォルダでは 1.98.0 を使う」という貼り紙を作る
Set-Content -Encoding ascii rust-toolchain.toml "[toolchain]`nchannel = `"1.98.0`""
"----"
rustup show active-toolchain
rustc --version
