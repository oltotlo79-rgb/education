REPLACEMENTS = [("""    // 消すときは、書いたときと同じ Path を指定する
    jar.remove(Cookie::build((COOKIE_NAME, "")).path("/").build());
""", """    // わざと間違えた版: 読むだけで、消さない
    let _ = Cookie::build((COOKIE_NAME, "")).path("/").build();
""")]
