import * as React from "react";
import { cn } from "@/lib/utils";

export const Checkbox = React.forwardRef<
  HTMLInputElement,
  React.InputHTMLAttributes<HTMLInputElement>
>(({ className, type = "checkbox", ...props }, ref) => {
  return (
    <input
      ref={ref}
      type={type}
      className={cn(
        "h-4 w-4 rounded border border-[var(--border)] accent-[var(--primary)]",
        className
      )}
      {...props}
    />
  );
});

Checkbox.displayName = "Checkbox";
