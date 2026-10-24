// R-6（問2）: views（画面の部品）は、DB とリポジトリに触らず、渡された値を表示するだけ
#[test]
fn r6_views_only_show_given_values() {
    let (modules, crates) = uses_of("views");
    for storage in ["db", "repository", "state"] {
        assert!(
            !modules.contains(storage),
            "views が {storage} を使っています"
        );
    }
    assert!(!crates.contains("libsql"), "views が libsql を使っています");
}
