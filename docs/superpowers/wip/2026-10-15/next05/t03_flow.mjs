import { psql, startRest, stopRest } from './h.mjs';
import { runTs } from './run.mjs';

// fresh books table, no grants (2026 default), RLS not yet enabled
await psql([
  `DROP TABLE IF EXISTS books CASCADE`,
  `CREATE TABLE books (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  title text NOT NULL,
  author text NOT NULL,
  publisher text,
  published_date date,
  rating integer CHECK (rating >= 1 AND rating <= 5),
  status text DEFAULT 'want_to_read' CHECK (status IN ('reading', 'completed', 'want_to_read')),
  notes text,
  cover_url text,
  created_at timestamptz DEFAULT NOW(),
  updated_at timestamptz DEFAULT NOW()
)`,
  `INSERT INTO books (title, author, status) VALUES ('こころ', '夏目漱石', 'reading'), ('人間失格', '太宰治', 'completed')`,
]);

console.log('===== (A) no GRANT =====');
await startRest();
runTs('01-check.ts');

console.log('===== (B) GRANT + RLS enabled, no policy =====');
await psql([
  `GRANT SELECT ON public.books TO anon`,
  `GRANT SELECT, INSERT, UPDATE, DELETE ON public.books TO authenticated`,
  `GRANT SELECT, INSERT, UPDATE, DELETE ON public.books TO service_role`,
  `ALTER TABLE books ENABLE ROW LEVEL SECURITY`,
]);
await startRest();
runTs('01-check.ts');

console.log('===== (C) with select policy =====');
await psql([
  `CREATE POLICY "Allow public read access" ON books FOR SELECT USING (true)`,
]);
await startRest();
runTs('01-check.ts');

console.log('===== (D) env missing =====');
runTs('01-check.ts', 'nonexistent.env');
stopRest();
