{books.map((book, index) => (
  <Link
    key={book.id}
    href={`/books/${book.id}`}
    className="group block"
    style={{
      animationDelay: `${index * 50}ms`,
    }}
  >
    <BookCard book={book} />
  </Link>
))}
