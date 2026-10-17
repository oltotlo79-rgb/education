SELECT COUNT(*) FROM books;
SELECT status, COUNT(*) as count FROM books GROUP BY status;
SELECT title, rating FROM books WHERE rating >= 4 ORDER BY rating DESC, published_date;
SELECT title, author FROM books WHERE status = 'reading' OR rating IS NULL ORDER BY published_date;
SELECT status, COUNT(*) AS count, AVG(rating) AS avg_rating FROM books GROUP BY status ORDER BY status;
SELECT title FROM books WHERE memo IS NULL;
