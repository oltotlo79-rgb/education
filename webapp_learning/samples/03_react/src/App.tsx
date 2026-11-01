import { useEffect, useState } from "react";
import { DrawingPad } from "./components/DrawingPad.tsx";
import { RequestForm } from "./components/RequestForm.tsx";
import { RequestList } from "./components/RequestList.tsx";

type Page = "list" | "new" | "drawing";

// URL の # の後ろ（#/new など）で、どの画面を出すかを決める
function pageFromHash(): Page {
  if (location.hash === "#/new") return "new";
  if (location.hash === "#/drawing") return "drawing";
  return "list";
}

export function App() {
  const [page, setPage] = useState<Page>(pageFromHash());
  const [message, setMessage] = useState("");

  useEffect(() => {
    const onChange = () => setPage(pageFromHash());
    window.addEventListener("hashchange", onChange);
    return () => window.removeEventListener("hashchange", onChange); // 片付け
  }, []);

  return (
    <>
      <header className="site-header">
        <strong>依頼・承認アプリ</strong>
        <nav>
          <a href="#/">自分の依頼</a>
          <a href="#/new">新しい依頼</a>
          <a href="#/drawing">図面</a>
        </nav>
      </header>
      <main>
        {message && <p className="message">{message}</p>}
        {page === "list" && <RequestList />}
        {page === "new" && (
          <RequestForm
            onCreated={(id) => {
              setMessage(`依頼 ${id} を受け付けました`);
              location.hash = "#/";
            }}
          />
        )}
        {page === "drawing" && <DrawingPad />}
      </main>
    </>
  );
}
