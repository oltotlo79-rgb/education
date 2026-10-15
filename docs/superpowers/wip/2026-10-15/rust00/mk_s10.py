import pathlib, sys
S = pathlib.Path(sys.argv[1])
t = (S / "u10_title_fixed.rs").read_text(encoding="utf-8")
old = """    #[test]
    fn eighty_one_characters_is_invalid() {
        let title = "あ".repeat(81);
        assert!(!is_valid_title(&title));
    }
"""
assert old in t
new = old + """
    #[test]
    fn spaces_only_title_is_invalid() {
        assert!(!is_valid_title("   "));
    }
"""
t = t.replace(old, new)
(S / "s10_spaces_fail.rs").write_text(t, encoding="utf-8")
t2 = t.replace("// タイトルの文字数を数える（文字の個数で数える）\nfn title_length(title: &str) -> usize {\n    title.chars().count()",
               "// タイトルの文字数を数える（前後の空白を取り除いてから数える）\nfn title_length(title: &str) -> usize {\n    title.trim().chars().count()")
assert t2 != t
(S / "s10_spaces_fixed.rs").write_text(t2, encoding="utf-8")
