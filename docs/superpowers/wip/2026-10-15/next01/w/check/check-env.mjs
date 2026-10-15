// 別のコマンドを実行して、その出力を受け取る道具を読み込む
import { execSync } from "node:child_process";

// 調べたいツールの一覧（表示名と、バージョンを確認するコマンド）
const tools = [
  { label: "Node.js", command: "node -v" },
  { label: "npm", command: "npm -v" },
  { label: "Git", command: "git --version" },
  { label: "VS Code", command: "code --version" },
];

let okCount = 0;

for (const tool of tools) {
  try {
    // コマンドを実行し、出力の1行目だけを取り出す
    const output = execSync(tool.command, { encoding: "utf8", stdio: "pipe" });
    const firstLine = output.trim().split("\n")[0];
    console.log(`[OK] ${tool.label.padEnd(8)} ${firstLine}`);
    okCount++;
  } catch {
    // コマンドが見つからない・失敗したときはここに来る
    console.log(`[NG] ${tool.label.padEnd(8)} 見つかりません（インストールと PATH を確認）`);
  }
}

// Node.js のメジャーバージョン（v24.22.0 なら 24）を数字で取り出して確認する
const major = Number(process.versions.node.split(".")[0]);
if (major >= 22) {
  console.log(`[OK] Node.js ${major} 系は本書の推奨（22 以上）を満たしています`);
} else {
  console.log(`[注意] Node.js ${major} 系は古すぎます。LTS 版に更新してください`);
}

console.log(`\n${okCount} / ${tools.length} 個のツールが使えます`);
