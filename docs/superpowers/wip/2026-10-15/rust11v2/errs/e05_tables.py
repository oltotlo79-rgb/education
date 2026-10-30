# 第11巻 UNIT 05 問1: 2つの nav に、名前（aria-label）を付ける（src/views.rs）。
NAV_LABELS = [
    (
        '        <header class="site-header">\n            <a class="brand" href=(href!(crate::routes::home))>"RustNote"</a>\n            <nav>\n',
        '        <header class="site-header">\n            <a class="brand" href=(href!(crate::routes::home))>"RustNote"</a>\n            <nav aria-label="メニュー">\n',
    ),
    (
        '        <nav class="pagination">\n',
        '        <nav class="pagination" aria-label="ページ送り">\n',
    ),
]
