import Link from "next/link";
import { headers } from "next/headers";
import { notFound } from "next/navigation";
import { AdminNav } from "@/app/admin/_components/nav";
import { requireAdminAccess } from "@/modules/subscription/interface/adminAuth";

const NAV_ITEMS = [
  { href: "/admin/aliases", label: "Aliases" },
  { href: "/admin/profiles", label: "Profiles" },
  { href: "/admin/sources", label: "Sources" },
  { href: "/admin/templates", label: "Templates" },
  { href: "/admin/logs/pulls", label: "Pull Logs" },
];

async function ensureAdminPageAccess(): Promise<void> {
  const incoming = await headers();
  const forwarded = new Headers();
  for (const [key, value] of incoming.entries()) {
    forwarded.set(key, value);
  }
  const request = new Request("https://control-plane.internal/admin", {
    method: "GET",
    headers: forwarded,
  });
  const auth = await requireAdminAccess(request);
  if (!auth.ok) {
    notFound();
  }
}

export default async function AdminLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  await ensureAdminPageAccess();
  return (
    <div className="min-h-screen">
      <header className="border-b border-[var(--border)] bg-[var(--card)]">
        <div className="mx-auto flex max-w-6xl flex-wrap items-center justify-between gap-3 px-4 py-3">
          <Link
            href="/admin/aliases"
            className="flex items-center gap-2 text-base font-semibold tracking-tight"
          >
            <img
              src="/brand-icon.svg"
              alt="MySub"
              width="24"
              height="24"
              className="h-6 w-6 shrink-0"
            />
            <span>MySub</span>
          </Link>
          <AdminNav items={NAV_ITEMS} />
        </div>
      </header>
      <main className="mx-auto w-full max-w-6xl px-4 py-6">{children}</main>
    </div>
  );
}
