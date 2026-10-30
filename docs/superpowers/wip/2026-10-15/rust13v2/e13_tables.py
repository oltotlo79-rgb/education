# 第13巻 UNIT 01 のわざと書き換える置き換え表（練習用のコピーの Cargo.toml・Cargo.lock）。

TOKIO_OLD_PIN = [
    ('tokio = { version = "=1.54.1", features = ["rt-multi-thread", "macros"] }',
     'tokio = { version = "=1.53.3", features = ["rt-multi-thread", "macros"] }'),
]
TOKIO_OLD_CARET = [
    ('tokio = { version = "=1.53.3", features = ["rt-multi-thread", "macros"] }',
     'tokio = { version = "1.53.3", features = ["rt-multi-thread", "macros"] }'),
]
NO_MACROS = [
    ('tokio = { version = "=1.54.1", features = ["rt-multi-thread", "macros"] }',
     'tokio = { version = "=1.54.1", features = ["rt-multi-thread"] }'),
]
TYPO = [
    ('tokio = { version = "=1.54.1", features = ["rt-multi-thread", "macros"] }',
     'tokio = { version = "=1.54.1", features = ["rt-multi-thread", "macro"] }'),
]
DOTENVY_OLD = [('dotenvy = "=0.15.7"', 'dotenvy = "=0.15.6"')]
CHECKSUM = [
    ('name = "dotenvy"\nversion = "0.15.7"\nsource = "registry+https://github.com/rust-lang/crates.io-index"\nchecksum = "1aaf95b3',
     'name = "dotenvy"\nversion = "0.15.7"\nsource = "registry+https://github.com/rust-lang/crates.io-index"\nchecksum = "0aaf95b3'),
]
