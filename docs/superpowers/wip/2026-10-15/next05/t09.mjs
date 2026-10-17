import { psql, startRest, stopRest } from './h.mjs';
import { runTs } from './run.mjs';
await psql([
  `DROP POLICY IF EXISTS "自分の本だけ読める" ON books`,
  `DROP POLICY IF EXISTS "自分の本として追加できる" ON books`,
  `DROP POLICY IF EXISTS "自分の本だけ更新できる" ON books`,
  `DROP POLICY IF EXISTS "自分の本だけ削除できる" ON books`,
  `ALTER TABLE books DROP COLUMN IF EXISTS user_id`,
  `CREATE POLICY "Allow public read access" ON books FOR SELECT USING (true)`,
  `CREATE POLICY "Allow public insert access" ON books FOR INSERT WITH CHECK (true)`,
  `CREATE POLICY "Allow public update access" ON books FOR UPDATE USING (true) WITH CHECK (true)`,
  `CREATE POLICY "Allow public delete access" ON books FOR DELETE USING (true)`,
  `GRANT INSERT, UPDATE, DELETE ON public.books TO anon`,
]);
await startRest();
runTs('08-ok-check.ts');
runTs('03-error.ts');
stopRest();
