// この crate（ライブラリ）の入口。どの module があるかをここで宣言する
pub mod domain;
mod repo;

// よく使う型を、crate のいちばん上からも使えるようにする（再公開）
pub use domain::note::{Note, NoteTitle};
pub use repo::NoteStore;
