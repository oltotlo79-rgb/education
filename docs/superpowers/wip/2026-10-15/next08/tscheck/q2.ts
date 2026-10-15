type Props = {
  params: Promise<{ id: string }>;
};

export async function BookDetailPage({ params }: Props) {
  const id = params.id;
  return id;
}
