import * as React from "react";
import { cn } from "@/lib/utils";

type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: "default" | "outline" | "destructive" | "ghost";
};

const variantClass: Record<NonNullable<ButtonProps["variant"]>, string> = {
  default:
    "bg-[var(--primary)] text-[var(--primary-foreground)] hover:brightness-110",
  outline:
    "border border-[var(--border)] bg-[var(--card)] text-[var(--foreground)] hover:bg-[var(--muted)]",
  destructive:
    "bg-[var(--danger)] text-[var(--danger-foreground)] hover:brightness-110",
  ghost: "text-[var(--foreground)] hover:bg-[var(--muted)]",
};

export const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant = "default", type = "button", ...props }, ref) => {
    return (
      <button
        ref={ref}
        type={type}
        className={cn(
          "inline-flex h-9 items-center justify-center rounded-md px-3 text-sm font-medium transition disabled:cursor-not-allowed disabled:opacity-50",
          variantClass[variant],
          className
        )}
        {...props}
      />
    );
  }
);

Button.displayName = "Button";
