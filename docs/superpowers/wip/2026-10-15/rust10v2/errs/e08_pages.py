REPLACEMENTS = [("let pages = self.total_count.div_ceil(u64::from(PER_PAGE)).max(1);", "let pages = (self.total_count / u64::from(PER_PAGE)).max(1);")]
