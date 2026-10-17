CREATE POLICY "Allow public read access" ON books FOR SELECT USING (true);
CREATE POLICY "Allow public insert access" ON books FOR INSERT WITH CHECK (true);
CREATE POLICY "Allow public update access" ON books FOR UPDATE USING (true) WITH CHECK (true);
CREATE POLICY "Allow public delete access" ON books FOR DELETE USING (true);
INSERT INTO books (title, author, status) VALUES ('こころ', '夏目漱石', 'reading'), ('人間失格', '太宰治', 'completed'), ('坊っちゃん', '夏目漱石', 'want_to_read'), ('銀河鉄道の夜', '宮沢賢治', 'want_to_read');
INSERT INTO auth.users (id, email) VALUES ('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', 'a@example.com'), ('bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', 'b@example.com') ON CONFLICT DO NOTHING;
