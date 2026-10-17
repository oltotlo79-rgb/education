const statusBadge = () => {
  switch (book.status) {
    case "reading":
      return <span className="badge-primary">読書中</span>;
    case "completed":
      return <span className="badge-success">読了</span>;
    case "want_to_read":
      return <span className="badge-warning">読みたい</span>;
    default:
      return null;
  }
};
