struct Note {
    done: bool,
}

impl Note {
    fn toggle(&mut self) {
        self.done = !self.done;
    }
}

fn main() {
    let note = Note { done: false };
    note.toggle();
    println!("{}", note.done);
}
