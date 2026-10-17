{book.rating !== undefined && book.rating > 0 && (
  <div className="mt-auto flex items-center gap-1 pt-2">
    {[1, 2, 3, 4, 5].map((star) => (
      <svg
        key={star}
        className={`h-3.5 w-3.5 ${
          star <= book.rating!
            ? "text-yellow-400"
            : "text-gray-300 dark:text-gray-600"
        }`}
        // ...
      >
        <path d="..." />
      </svg>
    ))}
  </div>
)}
