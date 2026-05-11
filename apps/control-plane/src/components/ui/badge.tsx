import * as React from "react";
import { cn } from "@/lib/utils";

type BadgeProps = React.HTMLAttributes<HTMLSpanElement> & {
  variant?: "default" | "muted" | "danger" | "success";
};

const variantClass: Record<NonNullable<BadgeProps["variant"]>, string> = {
  default: "bg-[var(--primary)] text-[var(--primary-foreground)]",
  muted: "bg-[var(--muted)] text-[var(--muted-foreground)]",
  danger: "bg-[#ffe2ea] text-[#9b1d3d]",
  success: "bg-[#e7f9ef] text-[#0c7a47]",
};

export function Badge({ className, variant = "muted", ...props }: BadgeProps) {
  return (
    <span
      className={cn(
        "inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium",
        variantClass[variant],
        className
      )}
      {...props}
    />
  );
}
