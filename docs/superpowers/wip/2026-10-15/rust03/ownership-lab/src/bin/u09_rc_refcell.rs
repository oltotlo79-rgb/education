use std::cell::RefCell;
use std::rc::Rc;

// 2つの部品が、同じ「操作ログ」を共有して書き込む
struct Button {
    label: String,
    log: Rc<RefCell<Vec<String>>>,
}

impl Button {
    fn click(&self) {
        // &self（読むだけの借用）しかないが、RefCell なので書き込める
        self.log.borrow_mut().push(format!("{} を押した", self.label));
    }
}

fn main() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let save = Button {
        label: String::from("保存"),
        log: Rc::clone(&log),
    };
    let cancel = Button {
        label: String::from("取消"),
        log: Rc::clone(&log),
    };

    save.click();
    cancel.click();
    save.click();

    println!("持ち主: {} 人", Rc::strong_count(&log));
    for line in log.borrow().iter() {
        println!("- {line}");
    }
}
