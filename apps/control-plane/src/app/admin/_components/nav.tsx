"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { cn } from "@/lib/utils";

type NavItem = {
  href: string;
  label: string;
};

export function AdminNav({ items }: { items: NavItem[] }) {
  const pathname = usePathname();
  return (
    <nav className="flex flex-wrap items-center gap-4 text-sm">
      {items.map((item) => {
        const active = pathname === item.href || pathname.startsWith(`${item.href}/`);
        return (
          <Link
            key={item.href}
            href={item.href}
            className={cn(
              "border-b border-transparent pb-0.5 text-[var(--muted-foreground)] transition-colors hover:text-[var(--foreground)]",
              active && "border-[var(--foreground)] text-[var(--foreground)]"
            )}
          >
            {item.label}
          </Link>
        );
      })}
    </nav>
  );
}
