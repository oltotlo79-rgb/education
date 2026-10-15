// 登録した人（ユーザー）
struct User {
    id: u32,
    email: String,
    // 注意: 学習用にパスワードをそのまま持っている。本物はハッシュだけを保存する
    password: String,
}

// ノート（誰のものかを user_id で覚えている）
struct Note {
    id: u32,
    user_id: u32,
    title: String,
}

// 認証: メールアドレスとパスワードが合っていれば、その人の id を返す
fn login(users: &[User], email: &str, password: &str) -> Option<u32> {
    for user in users {
        if user.email == email && user.password == password {
            return Some(user.id);
        }
    }
    None
}

// 認可: ログイン中の人（current_user）が持ち主であるノートだけを返す
fn find_note(notes: &[Note], current_user: u32, note_id: u32) -> Option<&Note> {
    notes.iter().find(|note| note.id == note_id)
}

fn main() {
    let users = vec![
        User {
            id: 1,
            email: String::from("hanako@example.com"),
            password: String::from("correct-horse"),
        },
        User {
            id: 2,
            email: String::from("taro@example.com"),
            password: String::from("battery-staple"),
        },
    ];
    let notes = vec![
        Note {
            id: 10,
            user_id: 1,
            title: String::from("牛乳を買う"),
        },
        Note {
            id: 11,
            user_id: 2,
            title: String::from("歯医者の予約"),
        },
    ];

    // 1. 認証（あなたは誰？）
    let wrong = login(&users, "hanako@example.com", "wrong-password");
    println!("[認証] 間違ったパスワード -> {wrong:?}");
    let hanako = login(&users, "hanako@example.com", "correct-horse");
    println!("[認証] 正しいパスワード   -> {hanako:?}");

    // 2. 認可（そのノートを見てよい？）
    let Some(current_user) = hanako else {
        println!("ログインできなかったので、ここで終わり");
        return;
    };
    for note_id in [10, 11] {
        match find_note(&notes, current_user, note_id) {
            Some(note) => println!("[認可] ノート {note_id} -> 200 OK「{}」", note.title),
            None => println!("[認可] ノート {note_id} -> 404 Not Found"),
        }
    }
}
