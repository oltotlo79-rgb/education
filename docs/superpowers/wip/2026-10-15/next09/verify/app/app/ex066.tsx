const [prefix, name] = token.includes(":")
  ? token.split(":")
  : ["", token];
