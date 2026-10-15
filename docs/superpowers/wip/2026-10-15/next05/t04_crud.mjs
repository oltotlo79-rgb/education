import { psql, startRest, stopRest } from './h.mjs';
import { runTs } from './run.mjs';

await psql([`GRANT INSERT, UPDATE, DELETE ON public.books TO anon`]);
await startRest();
console.log('===== (E) only SELECT policy -> insert =====');
runTs('02-insert.ts');

console.log('===== (F) all 4 dev policies =====');
await psql([
  `CREATE POLICY "Allow public insert access" ON books FOR INSERT WITH CHECK (true)`,
  `CREATE POLICY "Allow public update access" ON books FOR UPDATE USING (true) WITH CHECK (true)`,
  `CREATE POLICY "Allow public delete access" ON books FOR DELETE USING (true)`,
]);
await startRest();
runTs('02-insert.ts');
runTs('03-error.ts');
runTs('04-single.ts');
runTs('05-update.ts');
runTs('01-check.ts');
stopRest();
