OLD = """pub async fn find_own_note(cx: &Cx, owner: UserId, id: NoteId) -> Result<Note, AppError> {
    app_state(cx)
        .notes
        .find(owner, id)
        .await?
        .ok_or(AppError::NotFound)
}"""
NEW = """pub async fn find_own_note(cx: &Cx, owner: UserId, id: NoteId) -> Result<Note, AppError> {
    if let Some(note) = app_state(cx).notes.find(owner, id).await? {
        return Ok(note);
    }
    // わざと間違えた版: ほかの人のノートなら 403、本当に無いときだけ 404
    let conn = app_state(cx).db.connect().await.map_err(AppError::internal)?;
    let mut rows = conn
        .query("SELECT 1 FROM notes WHERE id = ?1", libsql::params![id.get()])
        .await
        .map_err(AppError::internal)?;
    if rows.next().await.map_err(AppError::internal)?.is_some() {
        Err(AppError::Forbidden)
    } else {
        Err(AppError::NotFound)
    }
}"""
REPLACEMENTS = [(OLD, NEW)]
