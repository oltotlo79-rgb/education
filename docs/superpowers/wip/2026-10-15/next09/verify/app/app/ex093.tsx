const createPageUrl = (page: number): string => {
  const params = new URLSearchParams(searchParams.toString());
  params.set("page", String(page));
  return `${basePath}?${params.toString()}`;
};
