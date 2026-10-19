-- ① N+1 を直すつもりで、LEFT JOIN ではなく JOIN と書いてしまった
SELECT u.email, COUNT(n.id) AS notes, COALESCE(SUM(n.done), 0) AS done
  FROM users AS u
  JOIN notes AS n ON n.user_id = u.id
 GROUP BY u.id
 ORDER BY u.id;

-- ② LEFT JOIN にしたが、COUNT(n.id) ではなく COUNT(*) と書いてしまった
SELECT u.email, COUNT(*) AS notes, COALESCE(SUM(n.done), 0) AS done
  FROM users AS u
  LEFT JOIN notes AS n ON n.user_id = u.id
 GROUP BY u.id
 ORDER BY u.id;
