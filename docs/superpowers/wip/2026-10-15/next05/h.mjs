// Harness: run SQL like psql (as superuser "postgres" = SQL Editor) and (re)start PostgREST.
import pg from 'pg';
import { spawn, execSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));

// display width (East Asian wide = 2) like psql
function w(s) {
  let n = 0;
  for (const ch of String(s)) {
    const c = ch.codePointAt(0);
    n += (c >= 0x1100 && (c <= 0x115f || (c >= 0x2e80 && c <= 0xa4cf) || (c >= 0xac00 && c <= 0xd7a3) || (c >= 0xf900 && c <= 0xfaff) || (c >= 0xfe30 && c <= 0xfe4f) || (c >= 0xff00 && c <= 0xff60) || (c >= 0xffe0 && c <= 0xffe6) || c >= 0x20000)) ? 2 : 1;
  }
  return n;
}
const padR = (s, n) => s + ' '.repeat(Math.max(0, n - w(s)));
const padL = (s, n) => ' '.repeat(Math.max(0, n - w(s))) + s;

function fmtVal(v) {
  if (v === null || v === undefined) return '';
  if (v instanceof Date) return v.toISOString();
  if (typeof v === 'object') return JSON.stringify(v);
  return String(v);
}

export function table(res) {
  const cols = res.fields.map((f) => f.name);
  const numeric = res.fields.map((f) => [20, 21, 23, 700, 701, 1700].includes(f.dataTypeID));
  const rows = res.rows.map((r) => cols.map((c) => fmtVal(r[c])));
  const widths = cols.map((c, i) => Math.max(w(c), ...rows.map((r) => w(r[i]))));
  const out = [];
  out.push(' ' + cols.map((c, i) => {
    const total = widths[i];
    const left = Math.floor((total - w(c)) / 2);
    return padR(' '.repeat(left) + c, total);
  }).join(' | ').replace(/\s+$/, ''));
  out.push(widths.map((x) => '-'.repeat(x + 2)).join('+'));
  for (const r of rows) {
    out.push(' ' + r.map((v, i) => (numeric[i] ? padL(v, widths[i]) : padR(v, widths[i]))).join(' | ').replace(/\s+$/, ''));
  }
  out.push(`(${rows.length} ${rows.length === 1 ? 'row' : 'rows'})`);
  return out.join('\n');
}

export async function connect() {
  const c = new pg.Client({ host: '127.0.0.1', port: 54329, user: 'postgres', database: 'postgres' });
  await c.connect();
  return c;
}

// Run statements one by one, printing psql-like output. stmts: array of SQL strings.
export async function psql(stmts, { echo = false, client } = {}) {
  stopRest();
  await new Promise((r) => setTimeout(r, 300));
  const c = client ?? (await connect());
  try {
    for (const s of stmts) {
      if (echo) console.log(s);
      try {
        const res = await c.query(s);
        const list = Array.isArray(res) ? res : [res];
        for (const r of list) {
          if (r.command === 'SELECT' && r.fields.length) console.log(table(r));
          else if (r.command === 'INSERT') console.log(`INSERT 0 ${r.rowCount}`);
          else if (['UPDATE', 'DELETE'].includes(r.command)) console.log(`${r.command} ${r.rowCount}`);
          else if (r.command) console.log(r.command === 'SET' ? 'SET' : r.command);
        }
      } catch (e) {
        console.log(`ERROR:  ${e.message}`);
        if (e.detail) console.log(`DETAIL:  ${e.detail}`);
        if (e.hint) console.log(`HINT:  ${e.hint}`);
      }
      console.log();
    }
  } finally {
    if (!client) await c.end();
  }
}

let rest;
export async function startRest() {
  try { execSync('taskkill /F /IM postgrest.exe', { stdio: 'ignore' }); } catch {}
  rest = spawn(path.join(here, 'bin', 'postgrest.exe'), [path.join(here, 'postgrest.conf')], { stdio: ['ignore', 'pipe', 'pipe'] });
  await new Promise((resolve, reject) => {
    let buf = '';
    const t = setTimeout(() => reject(new Error('postgrest timeout\n' + buf)), 20000);
    const on = (d) => {
      buf += d;
      if (/Schema cache loaded in/.test(buf)) { clearTimeout(t); resolve(); }
    };
    rest.stdout.on('data', on);
    rest.stderr.on('data', on);
  });
}
export function stopRest() {
  if (rest) rest.kill();
  try { execSync('taskkill /F /IM postgrest.exe', { stdio: 'ignore' }); } catch {}
}
