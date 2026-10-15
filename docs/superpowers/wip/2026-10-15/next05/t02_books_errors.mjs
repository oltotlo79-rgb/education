import { psql } from './h.mjs';

await psql([
  `DROP TABLE IF EXISTS books CASCADE`,
  `CREATE TABLE books (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  title text NOT NULL,
  author text NOT NULL,
  publisher text,
  published_date date,
  rating integer CHECK (rating >= 1 AND rating <= 5),
  status text DEFAULT 'want_to_read'
    CHECK (status IN ('reading', 'completed', 'want_to_read')),
  notes text,
  cover_url text,
  created_at timestamptz DEFAULT NOW(),
  updated_at timestamptz DEFAULT NOW()
)`,
  `CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql`,
  `CREATE TRIGGER update_books_updated_at
  BEFORE UPDATE ON books
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column()`,
  // errors
  `INSERT INTO books (title, author, rating) VALUES ('テスト', 'テスト', 10)`,
  `INSERT INTO books (title, author, status) VALUES ('テスト', 'テスト', 'done')`,
  `INSERT INTO books (title) VALUES ('著者なし')`,
  `SELECT * FROM books WHERE status = "reading"`,
  `SELECT title FROM books WHERE id = '2'`,
  `CREATE TABLE books (id uuid)`,
  // ok
  `INSERT INTO books (title, author) VALUES ('こころ', '夏目漱石')`,
  `SELECT title, author, status, rating FROM books`,
], { echo: true });
