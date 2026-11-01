// 依頼の状態と、状態を変える操作を「型」で決める

/** 依頼の状態。この 7 つ以外の文字は入れられない */
export type RequestStatus =
  | "回覧中"
  | "差し戻し中"
  | "引き戻し中"
  | "回覧完了"
  | "対応中"
  | "完了"
  | "取り下げ";

/** 状態を変える操作 */
export type Action =
  | "全員が承認"
  | "差し戻し"
  | "引き戻し"
  | "再依頼"
  | "取り下げ"
  | "担当を決める"
  | "引き渡し";

/** 今の状態と操作から、次の状態を返す。できない操作なら null を返す */
export function nextStatus(current: RequestStatus, action: Action): RequestStatus | null {
  switch (current) {
    case "回覧中":
      if (action === "全員が承認") return "回覧完了";
      if (action === "差し戻し") return "差し戻し中";
      if (action === "引き戻し") return "引き戻し中";
      return null;
    case "差し戻し中":
    case "引き戻し中":
      if (action === "再依頼") return "回覧中";
      if (action === "取り下げ") return "取り下げ";
      return null;
    case "回覧完了":
      return action === "担当を決める" ? "対応中" : null;
    case "対応中":
      return action === "引き渡し" ? "完了" : null;
    case "完了":
    case "取り下げ":
      return null;
    default: {
      // ここに来るのは、状態を足したのに case を書き忘れたとき。型の確かめでエラーになる
      const unreachable: never = current;
      return unreachable;
    }
  }
}

/** 一覧で使う札の色 */
export function badgeColor(status: RequestStatus): "白" | "オレンジ" | "青" | "グレー" {
  switch (status) {
    case "回覧中":
      return "白";
    case "差し戻し中":
    case "引き戻し中":
      return "オレンジ";
    case "回覧完了":
    case "対応中":
      return "青";
    case "完了":
    case "取り下げ":
      return "グレー";
  }
}
