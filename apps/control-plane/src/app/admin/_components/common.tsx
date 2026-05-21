import Link from "next/link";
import type { ComponentProps } from "react";
import { cn } from "@/lib/utils";
import { buttonVariantClass, type ButtonVariant } from "@/components/ui/button";

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

type LinkButtonProps = ComponentProps<typeof Link> & {
  variant?: ButtonVariant;
};

export function LinkButton({
  variant = "default",
  className,
  ...props
}: LinkButtonProps) {
  return (
    <Link
      {...props}
      className={cn(
        "inline-flex h-9 items-center justify-center rounded-md px-3 text-sm font-medium transition",
        buttonVariantClass[variant],
        className
      )}
    />
  );
}
