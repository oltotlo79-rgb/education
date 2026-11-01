// 練習用の「にせのサーバー」。本物のアプリでは、ここが fetch で Django や Topcoat を呼ぶ（06・08）
import type { Employee, NewRequest, RequestItem } from "./types.ts";

const wait = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

let requests: RequestItem[] = [
  { id: 4, title: "型の改造", detail: "取り付け穴を 2 か所追加", dueDate: "2026-11-03", status: "回覧中", approvers: ["101", "102"] },
  { id: 3, title: "棚の取り付け", detail: "倉庫の東側", dueDate: "2026-11-20", status: "差し戻し中", approvers: ["102"] },
  { id: 2, title: "部品の追加工", detail: "長さを 5mm 短く", dueDate: "2026-10-31", status: "回覧完了", approvers: ["101"] },
  { id: 1, title: "治具の製作", detail: "検査用", dueDate: "2026-10-25", status: "完了", approvers: ["101"] },
];

export const employees: Employee[] = [
  { no: "101", name: "山田 太郎", dept: "設計部 課長" },
  { no: "102", name: "佐藤 花子", dept: "設計部 係長" },
  { no: "201", name: "鈴木 一郎", dept: "製造部 課長" },
];

/** 依頼の一覧を取る（サーバーの返事を待つ時間のまねで、0.3 秒待つ） */
export async function fetchRequests(): Promise<RequestItem[]> {
  await wait(300);
  return [...requests];
}

/** 新しい依頼を送る */
export async function createRequest(input: NewRequest): Promise<RequestItem> {
  await wait(300);
  const created: RequestItem = { ...input, id: Math.max(...requests.map((r) => r.id)) + 1, status: "回覧中" };
  requests = [created, ...requests];
  return created;
}
