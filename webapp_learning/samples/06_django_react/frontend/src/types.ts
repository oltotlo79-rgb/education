/** サーバー（Django の API）とやり取りする物の形。serializers.py と合わせる */

export type Role = "member" | "worker" | "admin";
export type Me = { id: number; employee_no: string; name: string; role: Role };

export type Status = "circulating" | "returned" | "pulled_back" | "approved" | "in_progress" | "done" | "withdrawn";

export type RequestRow = {
  id: number;
  title: string;
  due_date: string;
  status: Status;
  status_label: string;
  waiting_for: string;
  created_at: string;
};

export type Shape =
  | { kind: "rect"; x: number; y: number; width: number; height: number }
  | { kind: "line"; x1: number; y1: number; x2: number; y2: number }
  | { kind: "text"; x: number; y: number; text: string };
export type Drawing = { version: 1; widthMm: number; heightMm: number; shapes: Shape[] };

export type StepRow = {
  round: number;
  order: number;
  approver_id: number;
  approver_name: string;
  approver_dept: string;
  decision: "waiting" | "approved" | "returned" | "cancelled";
  decision_label: string;
  decided_at: string | null;
};

export type RequestDetail = RequestRow & {
  detail: string;
  requester_name: string;
  assignee_name: string;
  drawing: Drawing | null;
  return_comments: string;
  round: number;
  steps: StepRow[];
  attachments: { id: number; original_name: string; size: number; uploaded_at: string }[];
  histories: { at: string; actor_name: string; action: string; note: string }[];
};

export type Employee = { id: number; name: string; dept: string; job_title: string };

export type RequestInput = {
  title: string;
  detail: string;
  due_date: string;
  approver_ids: number[];
  drawing: Drawing | null;
};
