"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { createClient } from "@/lib/supabase/client";

export default function NewBookPage() {
  const router = useRouter();
  const [title, setTitle] = useState("");

  async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    const supabase = createClient();
    await supabase.from("books").insert({ title, author: "", rating: 3 });
    router.push("/");
  }

  return (
    <form onSubmit={handleSubmit} className="p-6">
      <input value={title} onChange={(e) => setTitle(e.target.value)} />
      <button type="submit">登録</button>
    </form>
  );
}
