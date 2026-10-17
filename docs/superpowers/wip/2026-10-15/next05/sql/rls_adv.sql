ALTER TABLE books
  ADD COLUMN user_id uuid REFERENCES auth.users(id) DEFAULT auth.uid();
UPDATE books SET user_id = 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa' WHERE title IN ('こころ', '坊っちゃん');
UPDATE books SET user_id = 'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb' WHERE title IN ('人間失格', '銀河鉄道の夜');
CREATE POLICY "自分の本だけ読める" ON books FOR SELECT USING (auth.uid() = user_id);
CREATE POLICY "自分の本として追加できる" ON books FOR INSERT WITH CHECK (auth.uid() = user_id);
CREATE POLICY "自分の本だけ更新できる" ON books FOR UPDATE USING (auth.uid() = user_id) WITH CHECK (auth.uid() = user_id);
CREATE POLICY "自分の本だけ削除できる" ON books FOR DELETE USING (auth.uid() = user_id);
