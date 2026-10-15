type Props = {
  params: Promise<{ id: string }>;
};

export default async function EditBookPage({ params }: Props) {
  const { id } = await params;
  return (
    <main className="p-6">
      <h1 className="text-xl font-bold">書籍 {id} を編集</h1>
    </main>
  );
}
