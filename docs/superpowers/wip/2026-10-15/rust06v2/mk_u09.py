"""u09_runtime.rs から、エラー用（.discover() 無し）と問1の解答（checked_mul）を作る。"""
import sys

s = open("web_lab/src/bin/u09_runtime.rs", encoding="utf-8").read()
head = "//! 第6巻 UNIT 09: ブラウザの中で動く部分（signal・イベント・procedure）。\n//! 実行: topcoat asset bundle --bin u09_runtime のあとで cargo run --bin u09_runtime\n"
disc = "        // procedure（double）を集めて登録する\n        .discover()\n"
imp = "    router::{RouterBuilderDiscoverExt, module_router, page},\n"
for name, part in [("head", head), ("disc", disc), ("imp", imp)]:
    print(name, s.count(part))

nd = s.replace(head, "//! .discover() を書き忘れた\n").replace(disc, "").replace(imp, "    router::{module_router, page},\n")
if "discover()" in nd.split("\n", 1)[1]:
    sys.exit("discover が残っている")
open("errs/e09_nodiscover.rs", "w", encoding="utf-8", newline="\n").write(nd)

a = """#[procedure("/api/double")]
async fn double(value: usize) -> Result<usize> {
    Ok(value * 2)
}"""
b = """#[procedure("/api/double")]
async fn double(value: usize) -> Result<usize> {
    // ブラウザから来た値は、利用者の入力。入りきらないなら断る
    value
        .checked_mul(2)
        .ok_or_else(|| bad_request("2倍にすると大きすぎます").into())
}"""
q1 = s.replace(head, "//! 問1の解答: double で、2倍が usize に入りきらないときは 400 にする。\n//! 実行: topcoat asset bundle --bin u09_q1 のあとで cargo run --bin u09_q1\n")
if q1.count(a) != 1:
    sys.exit("double が見つからない")
q1 = q1.replace(a, b).replace(imp, "    router::{RouterBuilderDiscoverExt, error::bad_request, module_router, page},\n")
open("web_lab/src/bin/u09_q1.rs", "w", encoding="utf-8", newline="\n").write(q1)

st = open("errs/e09_struct.rs", encoding="utf-8").read()
fx = st.replace("#[derive(Clone)]\nstruct Cart", "#[record]\n#[derive(Clone)]\nstruct Cart").replace("    runtime::{RouterBuilderRuntimeExt, signal},", "    runtime::{RouterBuilderRuntimeExt, record, signal},")
open("web_lab/src/bin/e09_struct_fixed.rs", "w", encoding="utf-8", newline="\n").write(fx)
print("ok")
