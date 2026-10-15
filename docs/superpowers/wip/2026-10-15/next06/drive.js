// create-next-app を対話モードで起動し、プロンプトに順番にキー入力を送るドライバ
const { spawn } = require("child_process");
const fs = require("fs");
const path = require("path");

const cwd = path.join(__dirname, "work");
fs.mkdirSync(cwd, { recursive: true });
const appdata = path.join(__dirname, "fake-appdata");
fs.mkdirSync(appdata, { recursive: true });

const env = { ...process.env, APPDATA: appdata, NEXT_TELEMETRY_DISABLED: "1" };
delete env.CI;

const child = spawn("npx create-next-app@latest book-management", {
  cwd,
  env,
  shell: true,
  stdio: ["pipe", "pipe", "pipe"],
});

const steps = [
  [/recommended Next\.js defaults/, "\x1b[B\r"],
  [/use .*TypeScript/, "1\r"],
  [/Which linter/, "\r"],
  [/React Compiler/, "0\r"],
  [/Tailwind CSS/, "1\r"],
  [/src\/. directory/, "1\r"],
  [/App Router/, "1\r"],
  [/Cache Components/, "0\r"],
  [/customize the .*import alias/, "0\r"],
  [/AGENTS\.md/, "1\r"],
  [/help improve Next\.js/, "0\r"],
];
let idx = 0;
let buf = "";
const log = fs.createWriteStream(path.join(__dirname, "drive-raw.log"));

function onData(d) {
  const s = d.toString();
  log.write(s);
  process.stdout.write(s);
  buf += s;
  if (idx < steps.length && steps[idx][0].test(buf)) {
    const keys = steps[idx][1];
    idx++;
    buf = "";
    setTimeout(() => child.stdin.write(keys), 400);
  }
}
child.stdout.on("data", onData);
child.stderr.on("data", onData);
child.on("close", (code) => {
  log.end();
  console.log("\n[exit]", code, "steps done:", idx);
});
