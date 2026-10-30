# 第11巻 UNIT 04 で、RustNote のコピーの src/domain.rs に当てる書き換え。

# 直した版: 題に制御文字（NUL・タブなど）があれば断る
NO_CONTROL = [
    (
        """    #[error("タイトルは{max}文字以内にしてください（現在{len}文字）")]
    TooLong { len: usize, max: usize },
}
""",
        """    #[error("タイトルは{max}文字以内にしてください（現在{len}文字）")]
    TooLong { len: usize, max: usize },
    #[error("タイトルに制御文字（NUL・タブなどの見えない文字）は使えません")]
    Control,
}
""",
    ),
    (
        """        if len > Self::MAX_CHARS {
            return Err(TitleError::TooLong {
                len,
                max: Self::MAX_CHARS,
            });
        }
        Ok(NoteTitle(trimmed.to_string()))
""",
        """        if len > Self::MAX_CHARS {
            return Err(TitleError::TooLong {
                len,
                max: Self::MAX_CHARS,
            });
        }
        // SQLite の length() は NUL より前しか数えないので、NUL で始まる題は DB の CHECK で断られる
        if trimmed.chars().any(char::is_control) {
            return Err(TitleError::Control);
        }
        Ok(NoteTitle(trimmed.to_string()))
""",
    ),
]
