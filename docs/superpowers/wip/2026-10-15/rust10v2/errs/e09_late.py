REPLACEMENTS = [("""    let note = find_own_note(cx, user.id, id).await?;
    let flash = flash::take(cx);
""", """    let note = find_own_note(cx, user.id, id).await?;
    // わざと間違えた版: この画面では、お知らせを取り出さない
    let flash: Option<crate::flash::Flash> = None;
""")]
