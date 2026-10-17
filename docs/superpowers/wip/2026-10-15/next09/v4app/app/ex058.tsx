{pageNumbers.map((page, index) => {
  if (page === "ellipsis") {
    return <span key={`ellipsis-${index}`} ... aria-hidden="true">...</span>;
  }
  const isActive = page === currentPage;
  return isActive ? (
    <span key={page} className={activePageClass} aria-current="page" ...>{page}</span>
  ) : (
    <Link key={page} href={createPageUrl(page)} className={inactivePageClass} ...>{page}</Link>
  );
})}
