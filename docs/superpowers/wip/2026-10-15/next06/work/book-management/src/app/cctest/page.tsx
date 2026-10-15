import { cookies } from "next/headers";

export default async function Page() {
  const cookieStore = await cookies();
  return <p>Cookie の数: {cookieStore.getAll().length}</p>;
}
