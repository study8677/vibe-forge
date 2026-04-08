function normalizeQuery(query) {
  return String(query ?? '').trim().toLowerCase();
}

export function filterItemsByQuery(items, query) {
  const normalized = normalizeQuery(query);

  if (!normalized) {
    return [...items];
  }

  return items.filter((item) => {
    const haystack = [
      item.title,
      item.description,
      ...(Array.isArray(item.tags) ? item.tags : [])
    ]
      .join(' ')
      .toLowerCase();

    return haystack.includes(normalized);
  });
}
