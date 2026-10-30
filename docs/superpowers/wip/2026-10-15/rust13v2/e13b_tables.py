# 第13巻 UNIT 02 のわざと書き換える置き換え表。
TYPO_0003 = [("CREATE INDEX idx_sessions_expires ON sessions (expires_at);", "CREATE INDEX idx_sessions_expires ON sessions (expire_at);")]
PINNED_0002 = [("    body TEXT NOT NULL DEFAULT '',\n", "    body TEXT NOT NULL DEFAULT '',\n    pinned INTEGER NOT NULL DEFAULT 0,\n")]
ADD_0004 = [(
    """    Migration {
        version: "0003_sessions",
        sql: include_str!("../migrations/0003_sessions.sql"),
    },
];""",
    """    Migration {
        version: "0003_sessions",
        sql: include_str!("../migrations/0003_sessions.sql"),
    },
    Migration {
        version: "0004_note_pinned",
        sql: include_str!("../migrations/0004_note_pinned.sql"),
    },
];""",
)]
