const getPageNumbers = (): (number | "ellipsis")[] => {
  const pages: (number | "ellipsis")[] = [];
  const maxVisible = 5;
  if (totalPages <= maxVisible + 2) {
    for (let i = 1; i <= totalPages; i++) pages.push(i);
  } else {
    pages.push(1);
    if (currentPage <= 3) { pages.push(2, 3, 4); pages.push("ellipsis"); }
    else if (currentPage >= totalPages - 2) { pages.push("ellipsis"); pages.push(totalPages - 3, totalPages - 2, totalPages - 1); }
    else { pages.push("ellipsis"); pages.push(currentPage - 1, currentPage, currentPage + 1); pages.push("ellipsis"); }
    pages.push(totalPages);
  }
  return pages;
};
