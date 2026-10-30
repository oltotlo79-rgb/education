title: property test・fuzz・入力の読み取り
---checklist
- ☐ 「どんな入力でも成り立つ決まり」を関数で書き、proptest にでたらめな入力を作らせて確かめられる
- ☐ 失敗した入力が縮められる（shrink）様子と、種が .proptest-regressions に保存される仕組みを説明できる
- ☐ でたらめな入力を HTTP の入口に送り、500 にならないことを確かめ、見つかった入力（NUL で始まる題）の原因を追える
- ☐ cargo-fuzz（WSL の nightly）で fuzz target を書いて動かし、見つかった入力を cargo fuzz fmt で読める
- ☐ でたらめな試しは見つけない回もあることと、作り方を間違いの出やすい形に寄せる理由を言える
---glossary
| property test | どんな入力でも成り立つ決まりを、道具が作るたくさんの入力で確かめるテスト |
| proptest | Rust の property test のクレート。`proptest!` と strategy で入力を作る |
| strategy | 入力の作り方。正規表現の文字列・`any::<u32>()`・`prop_oneof!` など |
| shrink | 失敗した入力を、失敗したまま、小さく縮めること |
| `.proptest-regressions` | 失敗した入力の種を保存するファイル。Git に入れて共有する |
| fuzz・fuzzer | 入力を少しずつ変えて、とても多く試し、パニックを探すこと・その道具 |
| cargo-fuzz | libFuzzer を使う Rust の fuzz の道具。nightly で動く |
| NUL | 番号が 0 の文字。SQLite の `length()` は NUL より前だけを数える |
