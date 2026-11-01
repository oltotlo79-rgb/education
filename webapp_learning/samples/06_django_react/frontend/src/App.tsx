import { useCallback, useEffect, useState } from "react";
import { api, ApiError } from "./api.ts";
import { DetailPage } from "./pages/DetailPage.tsx";
import { FormPage } from "./pages/FormPage.tsx";
import { ListPage } from "./pages/ListPage.tsx";
import { LoginPage } from "./pages/LoginPage.tsx";
import { StatsPage } from "./pages/StatsPage.tsx";
import type { Me } from "./types.ts";

type Route =
  | { page: "list" }
  | { page: "new" }
  | { page: "detail"; id: number }
  | { page: "edit"; id: number }
  | { page: "stats" };

/** URL の # の後ろ（#/requests/5 など）から、出す画面を決める */
function parse(hash: string): Route {
  const m = hash.match(/^#\/requests\/(\d+)(\/edit)?$/);
  if (m) return m[2] ? { page: "edit", id: Number(m[1]) } : { page: "detail", id: Number(m[1]) };
  if (hash === "#/new") return { page: "new" };
  if (hash === "#/stats") return { page: "stats" };
  return { page: "list" };
}

export function App() {
  const [me, setMe] = useState<Me | null | undefined>(undefined); // undefined = 確かめ中、null = ログインしていない
  const [route, setRoute] = useState<Route>(parse(location.hash));

  const loadMe = useCallback(async () => {
    try {
      setMe(await api.me());
    } catch (e) {
      if (e instanceof ApiError && e.status === 403) setMe(null); // ログインしていない（60 分たった、など）
      else throw e;
    }
  }, []);

  useEffect(() => {
    api.csrf().then(loadMe); // 先に CSRF の札（クッキー）を受け取ってから
    const onChange = () => setRoute(parse(location.hash));
    window.addEventListener("hashchange", onChange);
    return () => window.removeEventListener("hashchange", onChange);
  }, [loadMe]);

  async function logout() {
    await api.logout();
    setMe(null);
  }

  return (
    <>
      <header className="site-header">
        <strong>依頼・承認アプリ</strong>
        {me && (
          <nav>
            <a href="#/">依頼の一覧</a>
            <a href="#/new">新しい依頼</a>
            <a href="#/stats">グラフ</a>
            <span className="who">{me.name} さん</span>
            <button className="link light" onClick={logout}>ログアウト</button>
          </nav>
        )}
      </header>
      <main>
        {me === undefined && <p>読み込み中…</p>}
        {me === null && <LoginPage onLogin={loadMe} />}
        {me && route.page === "list" && <ListPage me={me} />}
        {me && route.page === "new" && <FormPage />}
        {me && route.page === "edit" && <FormPage editId={route.id} />}
        {me && route.page === "detail" && <DetailPage id={route.id} me={me} />}
        {me && route.page === "stats" && <StatsPage />}
      </main>
    </>
  );
}
