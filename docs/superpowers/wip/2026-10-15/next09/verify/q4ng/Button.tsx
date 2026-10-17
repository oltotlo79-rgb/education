type Props = { variant: "primary" | "danger" };

export function Button({ variant }: Props) {
  const color = variant === "primary" ? "indigo" : "red";
  return <button className={`bg-${color}-600 text-white`}>OK</button>;
}
