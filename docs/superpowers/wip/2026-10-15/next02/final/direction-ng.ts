type Direction = "north" | "south" | "east" | "west";

const input: string = "up";

// 広い型（string）→ 狭い型（Direction）へ、確かめずに代入しようとした
const d2: Direction = input;
