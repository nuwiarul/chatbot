# Chat API (Phase 1)

Backend menyediakan endpoint sederhana untuk chat, dan meneruskan request ke llama.cpp (OpenAI-compatible).

## Endpoint

### `GET /health`
Tanpa auth. Untuk cek server & koneksi database.

Response:
```json
{ "ok": true, "db": true }
```

### `POST /v1/chat`
Wajib header:
- `Authorization: Bearer <API_KEY>`

Request:
```json
{
  "messages": [
    { "role": "user", "content": "Halo" }
  ],
  "temperature": 0.7,
  "max_tokens": 256
}
```

Response:
```json
{
  "message": { "role": "assistant", "content": "..." }
}
```

### `POST /v1/chat/stream` (SSE)
Wajib header:
- `Authorization: Bearer <API_KEY>`

Response: `text/event-stream` dengan event:
- `event: delta` berisi potongan teks (string)
- `event: done` dengan data `[DONE]`

Frontend disarankan memakai `fetch()` streaming (bukan `EventSource`) karena butuh header Authorization.

## Troubleshooting
- `401 Unauthorized`: pastikan `API_KEY` backend sama dengan `VITE_API_KEY` frontend (Bearer token).
- `502/500` dari backend: cek `LLAMA_BASE_URL` dan pastikan llama.cpp expose `/v1/chat/completions`.
- CORS error di browser: set `CORS_ALLOW_ORIGINS` di backend (mis. `http://localhost:5173`).
