import { renderAliasSubscription } from "@/modules/subscription/interface/subscriptionRender";

type RouteContext = {
  params: Promise<{ alias: string }>;
};

export async function GET(request: Request, context: RouteContext) {
  const { alias } = await context.params;
  return renderAliasSubscription(request, alias);
}

