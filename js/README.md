# Cross-platform KV Storage for subconverter-rs

This module provides a unified key-value storage interface for the subconverter-rs WebAssembly virtual file system (VFS). It supports:

- Vercel KV storage
- Netlify Blobs storage
- Local in-memory storage (for development)

## Setup

### Installation

For Vercel projects:
```bash
npm install @vercel/kv
```

For Netlify projects:
```bash
npm install @netlify/blobs
```

For local development, no additional packages are needed as an in-memory fallback is provided.

### Environment Configuration

#### Vercel KV

1. Create a Vercel KV database in your Vercel dashboard.
2. Make sure the following environment variables are available:
   - `KV_REST_API_URL`
   - `KV_REST_API_TOKEN`

#### Netlify Blobs

1. No special environment variables needed for Netlify Blobs.
2. Deploy to Netlify, and the system will detect the environment (`process.env.NETLIFY === 'true'`).

## Usage

The module automatically detects which platform you're on and adapts accordingly. All functions maintain the same interface regardless of the underlying storage provider.

The Rust code for `vercel_kv_vfs.rs` doesn't need any modifications - it will work with both Vercel KV and Netlify Blobs through these bindings.

### Available Functions

- `kv_get(key)` - Retrieve a value by key
- `kv_set(key, value)` - Store a value by key
- `kv_exists(key)` - Check if a key exists
- `kv_list(prefix)` - List all keys with a given prefix
- `kv_del(key)` - Delete a key

## Notes

- Binary data is handled seamlessly across platforms (Uint8Array in JavaScript, &[u8] in Rust)
- All operations are async
- Each platform has slightly different behavior but this adapter normalizes the behavior

## Error Handling

All functions include appropriate error handling with fallbacks or friendly errors. 