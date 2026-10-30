title: CI・依存・供給網・事故の対応
---checklist
- ☐ CI の fmt --check・clippy -D warnings・test --locked を手元で動かし、1つでも失敗したら止まる理由を言える
- ☐ Cargo.lock と --locked で、毎回同じ版の部品を使う理由を、失敗の例で説明できる
- ☐ cargo audit で部品の弱点を調べ、cargo tree -i でどこから来ているかをたどれる
- ☐ 直せない弱点に、使わない機能を外す・audit.toml に理由と見直す日を書いて受け入れる、と手を打てる
- ☐ 部品の弱点が見つかった日の事故の対応の順番を言え、cargo audit を毎日動かす理由を言える
---glossary
| CI | 変更のたびに、決まった確かめを自動で毎回動かすこと |
| `--locked` | Cargo.lock を書き換えずに、記録どおりの版で組み立てる指定 |
| 供給網（サプライチェーン） | 自分のプログラムが使う部品と、その部品が使う部品のつながり |
| RustSec | Rust のクレートの弱点や、手入れが止まった知らせを集めたデータベース |
| cargo-audit | Cargo.lock の部品を、RustSec と照らし合わせる道具 |
| `cargo tree -i` | ある部品を、どの部品が使っているかを、逆向きにたどる |
| feature | クレートの機能のスイッチ。使わない機能を外すと、その部品も組み立てない |
| `audit.toml` | cargo audit の設定。受け入れる勧告の番号を、理由つきで書く |
