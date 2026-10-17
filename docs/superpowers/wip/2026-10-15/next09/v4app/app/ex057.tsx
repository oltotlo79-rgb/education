{isFirstPage ? (
  <span className={disabledNavClass} aria-disabled="true"> ... 前へ </span>
) : (
  <Link href={createPageUrl(currentPage - 1)} className={enabledNavClass} aria-label="前のページへ"> ... 前へ </Link>
)}
