type Props = { variant: "primary" | "danger" };

// 完成したクラス名を辞書に書いておく
const variantClasses = {
  primary: "bg-indigo-600",
  danger: "bg-red-600",
};

export function Button({ variant }: Props) {
  return <button className={`${variantClasses[variant]} text-white`}>OK</button>;
}
