{book.thumbnailUrl ? (
  <img
    src={book.thumbnailUrl}
    alt={`「${book.title}」の表紙`}
    className="h-full w-full object-cover ... group-hover:scale-105"
    loading="lazy"
  />
) : (
  /* プレースホルダー（画像がない場合の代替表示） */
  <div className="flex h-full w-full ...">
    {/* No Image アイコン */}
  </div>
)}
