// run a playground script exactly as the reader would: node --env-file=.env.local <file>
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
const here = path.dirname(fileURLToPath(import.meta.url));
export function runTs(file, envFile = '.env.local') {
  const r = spawnSync(process.execPath, [`--env-file=${envFile}`, file], {
    cwd: path.join(here, 'supabase-playground'),
    encoding: 'utf8',
  });
  console.log(`$ node --env-file=${envFile} ${file}`);
  process.stdout.write(r.stdout);
  process.stdout.write(r.stderr);
  console.log(`[exit ${r.status}]\n`);
}
