OLD = """        let conn = self.db.connect().await?;
        // done を 0 ⇔ 1 に反転する。中身が変わるので version も上げる
        let sql = format!(
            "UPDATE notes
                SET done = 1 - done, version = version + 1, updated_at = datetime('now')
              WHERE id = ?1 AND user_id = ?2
              RETURNING {NOTE_COLUMNS}"
        );
        let mut rows = conn
            .query(&sql, libsql::params![id.get(), owner.get()])
            .await?;"""
NEW = """        let conn = self.db.connect().await?;
        // わざと間違えた版: 今の done を読み、Rust で反転してから、書き込む
        let current = self.find(owner, id).await?.ok_or(RepoError::NotFound)?;
        // DB が遠い（Turso など）ときの、読むと書くの間の待ち時間の代わり
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let sql = format!(
            "UPDATE notes
                SET done = ?1, version = version + 1, updated_at = datetime('now')
              WHERE id = ?2 AND user_id = ?3
              RETURNING {NOTE_COLUMNS}"
        );
        let mut rows = conn
            .query(&sql, libsql::params![!current.done, id.get(), owner.get()])
            .await?;"""
REPLACEMENTS = [(OLD, NEW)]
