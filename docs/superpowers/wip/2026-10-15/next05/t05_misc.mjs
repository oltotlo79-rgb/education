import { startRest, stopRest } from './h.mjs';
import { runTs } from './run.mjs';
import { sign } from './keys.mjs';
import fs from 'node:fs';
import { spawnSync } from 'node:child_process';
import path from 'node:path';

const pg = path.resolve('supabase-playground');
// good / bad env files for the key checker (fake keys)
fs.writeFileSync(path.join(pg, 'good.env'),
  'NEXT_PUBLIC_SUPABASE_URL=https://abcdefghijklmnop.supabase.co\n' +
  'NEXT_PUBLIC_SUPABASE_ANON_KEY=sb_publishable_Ab12Cd34Ef56Gh78\n' +
  'SUPABASE_SECRET_KEY=sb_secret_Zy98Xw76Vu54Ts32\n');
fs.writeFileSync(path.join(pg, 'bad.env'),
  'NEXT_PUBLIC_SUPABASE_URL=https://abcdefghijklmnop.supabase.co\n' +
  'NEXT_PUBLIC_SUPABASE_ANON_KEY=' + sign({ role: 'anon' }) + '\n' +
  'NEXT_PUBLIC_SUPABASE_SERVICE_ROLE_KEY=' + sign({ role: 'service_role' }) + '\n');

await startRest();
runTs('06-ng-ignore-error.ts');
stopRest();
runTs('07-key-check.ts', 'good.env');
runTs('07-key-check.ts', 'bad.env');

// run without --env-file
const r = spawnSync(process.execPath, ['01-check.ts'], { cwd: pg, encoding: 'utf8' });
console.log('$ node 01-check.ts');
process.stdout.write(r.stdout + r.stderr);
console.log(`[exit ${r.status}]`);
