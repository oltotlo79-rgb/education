use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    name: String,
    // 親も子も強い参照で持ってしまうと……
    other: RefCell<Option<Rc<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("片付け: {}", self.name);
    }
}

fn main() {
    let folder = Rc::new(Node {
        name: String::from("フォルダ"),
        other: RefCell::new(None),
    });
    let memo = Rc::new(Node {
        name: String::from("メモ"),
        other: RefCell::new(Some(Rc::clone(&folder))),
    });
    *folder.other.borrow_mut() = Some(Rc::clone(&memo));

    println!("folder: 強 {}", Rc::strong_count(&folder));
    println!("memo:   強 {}", Rc::strong_count(&memo));
    println!("main 終了");
}
