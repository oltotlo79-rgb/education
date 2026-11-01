import type { RequestStatus } from "../types.ts";

// 状態ごとの CSS の class（01 HTML と CSS の 4 章と同じ色）
const className: Record<RequestStatus, string> = {
  回覧中: "badge-circulating",
  差し戻し中: "badge-returned",
  引き戻し中: "badge-returned",
  回覧完了: "badge-approved",
  対応中: "badge-approved",
  完了: "badge-done",
  取り下げ: "badge-done",
};

/** 状態の札。props で状態を受け取って、色を変える */
export function StatusBadge({ status }: { status: RequestStatus }) {
  return <span className={`badge ${className[status]}`}>{status}</span>;
}
