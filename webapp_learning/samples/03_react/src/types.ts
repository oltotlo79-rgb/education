/** 依頼の状態（02 TypeScript の基礎と同じ） */
export type RequestStatus = "回覧中" | "差し戻し中" | "引き戻し中" | "回覧完了" | "対応中" | "完了" | "取り下げ";

/** 依頼 1 件 */
export type RequestItem = {
  id: number;
  title: string;
  detail: string;
  dueDate: string;
  status: RequestStatus;
  approvers: string[];
};

/** 新しい依頼を出すときに送る物（番号と状態は、サーバーが決める） */
export type NewRequest = Omit<RequestItem, "id" | "status">;

/** 回覧者の候補 */
export type Employee = { no: string; name: string; dept: string };
