export type BookStatus = "reading" | "completed" | "want_to_read";

type StatusBadgeProps = {
  status: BookStatus;
};

const statusConfig: Record<
  BookStatus,
  { label: string; bgColor: string; textColor: string }
> = {
  reading: {
    label: "読書中",
    bgColor: "bg-blue-100",
    textColor: "text-blue-800",
  },
  completed: {
    label: "読了",
    bgColor: "bg-green-100",
    textColor: "text-green-800",
  },
  want_to_read: {
    label: "積読",
    bgColor: "bg-gray-100",
    textColor: "text-gray-700",
  },
};
