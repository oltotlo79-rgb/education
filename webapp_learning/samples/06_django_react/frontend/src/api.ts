// Django の API を呼ぶ。03 の「にせのサーバー」を、本物に置きかえた物
import type { Employee, Me, RequestDetail, RequestInput, RequestRow } from "./types.ts";

/** API が断ったとき。detail はそのまま画面に出せる日本語 */
export class ApiError extends Error {
  status: number;
  constructor(status: number, message: string) {
    super(message);
    this.status = status;
  }
}

function csrfToken(): string {
  const m = document.cookie.match(/(?:^|; )csrftoken=([^;]+)/);
  return m ? decodeURIComponent(m[1]) : "";
}

/** DRF のエラーの JSON（{"title": ["…"]} や {"detail": "…"}）を、1 つの文にする */
function messageOf(data: unknown, status: number): string {
  if (data && typeof data === "object") {
    const values = Object.values(data as Record<string, unknown>).flat();
    if (values.length > 0) return values.map(String).join(" ");
  }
  return `エラー ${status}`;
}

async function call<T>(path: string, init: RequestInit = {}): Promise<T> {
  const method = init.method ?? "GET";
  const headers: Record<string, string> = method === "GET" ? {} : { "X-CSRFToken": csrfToken() };
  if (init.body && !(init.body instanceof FormData)) headers["Content-Type"] = "application/json";
  const res = await fetch(path, { ...init, method, headers, credentials: "same-origin" });
  if (res.status === 204) return undefined as T;
  const data = await res.json().catch(() => ({}));
  if (!res.ok) throw new ApiError(res.status, messageOf(data, res.status));
  return data as T;
}

const post = <T>(path: string, body?: unknown) =>
  call<T>(path, { method: "POST", body: body instanceof FormData ? body : JSON.stringify(body ?? {}) });

export const api = {
  csrf: () => call<void>("/api/csrf/"),
  me: () => call<Me>("/api/me/"),
  login: (employee_no: string, password: string) => post<{ name: string }>("/api/login/", { employee_no, password }),
  logout: () => post<{ ok: boolean }>("/api/logout/"),
  requests: (mine: boolean) => call<RequestRow[]>(`/api/requests/${mine ? "?mine=1" : ""}`),
  request: (id: number) => call<RequestDetail>(`/api/requests/${id}/`),
  create: (input: RequestInput) => post<{ id: number }>("/api/requests/", input),
  action: (id: number, action: string, input?: RequestInput) => post<{ ok: boolean }>(`/api/requests/${id}/${action}/`, input),
  upload: (id: number, file: File) => {
    const form = new FormData();
    form.append("file", file);
    return post<{ id: number }>(`/api/requests/${id}/attachments/`, form);
  },
  employees: (q: string) => call<Employee[]>(`/api/employees/?q=${encodeURIComponent(q)}`),
  stats: () =>
    call<{ by_status: { status: string; count: number }[]; done_by_assignee: { name: string; count: number }[] }>("/api/stats/"),
};
