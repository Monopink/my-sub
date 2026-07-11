<p align="center">
  <img src="apps/control-plane/public/brand-icon.svg" alt="MySub" width="72" height="72" />
</p>

# MySub

Subscription aggregation service for multi-device use, based on `subconverter-rs`.

## Runtime Layout

- `apps/control-plane`: Next.js control plane (OpenNext on Cloudflare Workers)
- `workers/converter`: Rust converter Worker (stateless, no KV binding)
- Dynamic data: Cloudflare KV (`SUB_KV`, control-plane only)
- Internal call: `control-plane` -> `converter` via Service Binding (`CONVERTER_SERVICE`)

## Public/Admin Entry

- Public subscription entry: `GET /{alias}`
- Admin API: `/api/admin/*`
- Admin UI: `/admin/*`

## Deploy (Cloudflare)

### Recommended: GitHub Actions Auto Deploy

Workflow file:

- `.github/workflows/deploy-cloudflare.yml`

Deploy behavior:

1. Trigger on push to `main` (converter/control-plane/core files changed), or manual `workflow_dispatch`.
2. Deploy sequence is fixed: `converter` -> `control-plane`.
3. Before deploying `control-plane`, workflow syncs runtime secrets via `wrangler secret put`.

Required GitHub Actions Secrets:

1. `CLOUDFLARE_API_TOKEN`
2. `CLOUDFLARE_ACCOUNT_ID`
3. `SUB_KV_NAMESPACE_ID`
4. `SUB_KV_PREVIEW_NAMESPACE_ID`
5. `CONFIG_ASSET_BASE_URL`
6. `SUBCONVERTER_TIMEOUT_MS`
7. `PULL_LOG_RETENTION_DAYS`
8. `CF_ACCESS_TEAM_DOMAIN`
9. `CF_ACCESS_AUD`
10. `CF_ACCESS_ISSUER` (optional)

Notes:

1. `CLOUDFLARE_API_TOKEN` should include Worker/KV/R2 edit permissions for this account.
2. `SUB_KV_NAMESPACE_ID` / `SUB_KV_PREVIEW_NAMESPACE_ID` are injected into `apps/control-plane/wrangler.jsonc` during CI.
3. `services.CONVERTER_SERVICE` in `wrangler.jsonc` should point to `my-sub-converter`.

### Manual Deploy (Fallback)

Converter:

```bash
cd workers/converter
npm install
npx wrangler deploy
```

Control plane:

```bash
cd apps/control-plane
npm install
npm run build:cf
npm run deploy:cf
```

## Required Bindings / Variables

### Service & Storage Bindings

In `apps/control-plane/wrangler.jsonc`:

- `services.CONVERTER_SERVICE` -> `my-sub-converter`
- `kv_namespaces.SUB_KV` -> your KV namespace IDs

### Environment Variables

```bash
CONFIG_ASSET_BASE_URL=
SUBCONVERTER_TIMEOUT_MS=15000
PULL_LOG_RETENTION_DAYS=30

CF_ACCESS_TEAM_DOMAIN=
CF_ACCESS_AUD=
CF_ACCESS_ISSUER=
```

## License

GPL-3.0+ (same as upstream project license in this repository).
