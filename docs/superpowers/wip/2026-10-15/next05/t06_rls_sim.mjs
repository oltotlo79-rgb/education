import { psql, connect } from './h.mjs';

const A = 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa';
const B = 'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb';

await psql([
  `INSERT INTO auth.users (id, email) VALUES ('${A}', 'a@example.com'), ('${B}', 'b@example.com') ON CONFLICT DO NOTHING`,
  `ALTER TABLE books ADD COLUMN IF NOT EXISTS user_id uuid REFERENCES auth.users(id) DEFAULT auth.uid()`,
  `UPDATE books SET user_id = CASE WHEN title IN ('こころ', '坊っちゃん') THEN '${A}'::uuid ELSE '${B}'::uuid END`,
  `CREATE POLICY "自分の本だけ読める" ON books FOR SELECT USING (auth.uid() = user_id)`,
]);

const sim = (who) => [
  `BEGIN`,
  `SET LOCAL ROLE authenticated`,
  `SET LOCAL request.jwt.claim.sub = '${who}'`,
  `SELECT title FROM books ORDER BY title`,
  `ROLLBACK`,
];

console.log('===== NG: dev policy still there, as A =====');
await psql(sim(A), { echo: true });

await psql([
  `DROP POLICY "Allow public read access" ON books`,
]);
console.log('===== OK: after DROP, as A =====');
await psql(sim(A), { echo: true });
console.log('===== OK: after DROP, as B =====');
await psql(sim(B));
console.log('===== anon =====');
await psql([`BEGIN`, `SET LOCAL ROLE anon`, `SELECT title FROM books ORDER BY title`, `ROLLBACK`], { echo: true });
