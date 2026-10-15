import { createClient } from "@/lib/supabase/server";

type Props = {
  params: Promise<{ id: string }>;
};

export default async function BookDetailPage({ params }: Props) {
  const { id } = await params;
  const supabase = await createClient();
  const { data: book } = await supabase
    .from("books")
    .select("*")
    .eq("id", id)
    .single();

  return (
    <main className="p-6">
      <h1 className="text-xl font-bold">{book?.title ?? "見つかりません"}</h1>
    </main>
  );
}
