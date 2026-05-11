import { SubscriptionService } from "@/modules/subscription/application/usecases";
import { KvSubscriptionRepository } from "@/modules/subscription/infrastructure/kvRepository";
import { kvBackendName } from "@/modules/subscription/infrastructure/kvClient";

const repository = new KvSubscriptionRepository();
const service = new SubscriptionService(repository);

export function getSubscriptionService(): SubscriptionService {
  return service;
}

export function getBackendInfo(): { kvBackend: string } {
  return { kvBackend: kvBackendName() };
}

