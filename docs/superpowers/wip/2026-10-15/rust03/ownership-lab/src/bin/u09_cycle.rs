use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    name: String,
    // 子は強い参照で持つ（子の持ち主になる）
    children: RefCell<Vec<Rc<Node>>>,
    // 親は弱い参照で持つ（持ち主の人数に数えない）
    parent: RefCell<Weak<Node>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("片付け: {}", self.name);
    }
}

fn node(name: &str) -> Rc<Node> {
    Rc::new(Node {
        name: name.to_string(),
        children: RefCell::new(Vec::new()),
        parent: RefCell::new(Weak::new()),
    })
}

fn main() {
    let folder = node("フォルダ");
    let memo = node("メモ");
    folder.children.borrow_mut().push(Rc::clone(&memo));
    *memo.parent.borrow_mut() = Rc::downgrade(&folder);

    // Weak から本体を使うときは upgrade で「まだ生きているか」を確かめる
    if let Some(parent) = memo.parent.borrow().upgrade() {
        println!("{} の親は {}", memo.name, parent.name);
    }
    println!(
        "folder: 強 {} / 弱 {}",
        Rc::strong_count(&folder),
        Rc::weak_count(&folder)
    );
    println!(
        "memo:   強 {} / 弱 {}",
        Rc::strong_count(&memo),
        Rc::weak_count(&memo)
    );
    println!("main 終了");
}
