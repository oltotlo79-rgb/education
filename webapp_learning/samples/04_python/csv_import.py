"""社員の CSV を読み込み、行ごとに確かめる（取り込みの練習）"""

import csv
import re
import sys
from dataclasses import dataclass
from pathlib import Path


@dataclass
class Employee:
    employee_no: str
    name: str
    dept: str
    email: str


def read_employees(path: Path) -> tuple[list[Employee], list[str]]:
    """正しい行と、エラーの文の一覧を返す"""
    employees: list[Employee] = []
    errors: list[str] = []
    seen: set[str] = set()
    # utf-8-sig: Excel の「CSV UTF-8」が先頭に付ける印（BOM）を読み飛ばす
    with path.open(encoding="utf-8-sig", newline="") as f:
        for line_no, row in enumerate(csv.DictReader(f), start=2):  # 1 行目は見出し
            no = (row.get("従業員番号") or "").strip()
            name = (row.get("氏名") or "").strip()
            dept = (row.get("所属") or "").strip()
            email = (row.get("メールアドレス") or "").strip()
            if not re.fullmatch(r"\d{6}", no):
                errors.append(f"{line_no} 行目: 従業員番号は 6 桁の数字にしてください（{no!r}）")
                continue
            if no in seen:
                errors.append(f"{line_no} 行目: 従業員番号 {no} が 2 回出てきます")
                continue
            if not name:
                errors.append(f"{line_no} 行目: 氏名が空です")
                continue
            if "@" not in email:
                errors.append(f"{line_no} 行目: メールアドレスの形が違います（{email!r}）")
                continue
            seen.add(no)
            employees.append(Employee(no, name, dept, email))
    return employees, errors


if __name__ == "__main__":
    for file_name in sys.argv[1:]:
        print(f"== {file_name}")
        try:
            ok, ng = read_employees(Path(file_name))
        except UnicodeDecodeError as e:
            print("文字コードが UTF-8 ではありません。Excel で「CSV UTF-8」として保存し直してください。", e.reason)
            continue
        print(f"取り込める行: {len(ok)} 件")
        for emp in ok:
            print("  ", emp.employee_no, emp.name, emp.dept)
        for message in ng:
            print("  ×", message)
