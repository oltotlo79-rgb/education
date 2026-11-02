// 付録のサンプルを表示する入り口。appendix.html?demo=x01 のように、1 つだけ表示もできる
import { StrictMode, type ReactNode } from "react";
import { createRoot } from "react-dom/client";
import { X01Counter } from "./X01Counter.tsx";
import { X02List } from "./X02List.tsx";
import { X03Form } from "./X03Form.tsx";
import { X04Fetch } from "./X04Fetch.tsx";
import { X05Props } from "./X05Props.tsx";
import "./appendix.css";

const demos: Record<string, { title: string; view: ReactNode }> = {
  x01: { title: "B-08 useState とイベント", view: <X01Counter /> },
  x02: { title: "B-09 一覧・絞り込み", view: <X02List /> },
  x03: { title: "B-10 フォーム", view: <X03Form /> },
  x04: { title: "B-11 useEffect と fetch", view: <X04Fetch url="/appendix_requests.json" /> },
  x04err: { title: "B-11 useEffect と fetch（失敗）", view: <X04Fetch url="/no_such_file.json" /> },
  x05: { title: "B-12 部品に分ける", view: <X05Props /> },
};

const only = new URLSearchParams(location.search).get("demo");
const keys = only && only in demos ? [only] : Object.keys(demos);

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    {keys.map((k) => (
      <section key={k} className="x-demo">
        <h2>{demos[k].title}</h2>
        {demos[k].view}
      </section>
    ))}
  </StrictMode>,
);
