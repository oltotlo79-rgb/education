REPLACEMENTS = [("""            <form method="post" action=(href!(delete::delete, Id(id)))>
                <button type="submit" class="danger">"削除"</button>
            </form>""", """            <a class="button danger" href=(href!(delete::delete, Id(id)))>"削除"</a>""")]
