BEGIN;
SET LOCAL ROLE authenticated;
SET LOCAL request.jwt.claim.sub = 'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb';
SELECT title, auth.uid() = user_id AS is_mine FROM books ORDER BY title;
ROLLBACK;
