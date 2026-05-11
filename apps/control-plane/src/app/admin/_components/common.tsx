import { cn } from "@/lib/utils";

export function PageTitle({
  title,
  description,
}: {
  title: string;
  description: string;
}) {
  return (
    <div className="mb-4">
      <h1 className="text-xl font-bold">{title}</h1>
      <p className="text-sm text-[var(--muted-foreground)]">{description}</p>
    </div>
  );
}

export function InlineError({ message }: { message: string }) {
  if (!message) {
    return null;
  }
  return (
    <p className={cn("rounded-md bg-[#ffe2ea] px-3 py-2 text-sm text-[#9b1d3d]")}>
      {message}
    </p>
  );
}

export function InlineSuccess({ message }: { message: string }) {
  if (!message) {
    return null;
  }
  return (
    <p className="rounded-md bg-[#e7f9ef] px-3 py-2 text-sm text-[#0c7a47]">{message}</p>
  );
}
