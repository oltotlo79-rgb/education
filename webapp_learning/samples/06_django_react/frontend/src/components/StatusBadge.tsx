import type { Status } from "../types.ts";

// 状態の値（英字）→ 札の色の class（01 HTML と CSS の 4 章と同じ色）
const className: Record<Status, string> = {
  circulating: "badge-circulating",
  returned: "badge-returned",
  pulled_back: "badge-returned",
  approved: "badge-approved",
  in_progress: "badge-approved",
  done: "badge-done",
  withdrawn: "badge-done",
};

export function StatusBadge({ status, label }: { status: Status; label: string }) {
  return <span className={`badge ${className[status]}`}>{label}</span>;
}
