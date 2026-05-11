# Converter Worker Settings

The converter worker uses an embedded pref file by default:

- `base/config/my-sub-pref.ini`

No secret is required for normal deployment.

## Optional runtime override

If needed, you can override the embedded pref with:

- `SUBCONVERTER_PREF_CONTENT`

From `workers/converter`:

```bash
wrangler secret put SUBCONVERTER_PREF_CONTENT < ../../base/config/my-sub-pref.ini
```

When this secret is set and non-empty, it takes precedence over the embedded pref.
