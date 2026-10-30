//! RustNote の検査済みの型（src/domain.rs）を、そのまま取り込む。
//! domain.rs は標準ライブラリと thiserror だけを使うので、DB や Topcoat なしで動かせる。

#[path = "../../rustnote/src/domain.rs"]
pub mod domain;
