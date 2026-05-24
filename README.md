# Chatbot + RAG (SvelteKit + Axum + pgvector + llama.cpp)

Project belajar membangun chatbot lokal (llama.cpp OpenAI-compatible) + RAG + guardrails.

## Struktur repo
- `infra/` docker-compose & config service (Postgres + pgvector)
- `docs/` dokumentasi dan checklist per fase
- `backend/` Rust (Axum + SQLx)
- `frontend/` SvelteKit 5 + Tailwind 4 + Vite (Bun)

## Prasyarat
- Docker Desktop (untuk Postgres) atau Postgres lokal
- Rust toolchain
- Bun
- llama.cpp `llama-server` berjalan (OpenAI-compatible)

## Jalankan (dev)
1) Infra (Postgres + pgvector)
- `cd F:\chatbot\infra`
- `docker compose up -d`

Default DB URL: `postgres://chatbot:chatbot@127.0.0.1:5432/chatbot`

2) Backend
- `cd F:\chatbot\backend`
- copy `F:\chatbot\backend\.env.example` → `.env` lalu sesuaikan
- `cargo run`

Endpoint:
- `GET http://127.0.0.1:8080/health`
- `POST http://127.0.0.1:8080/v1/chat` (butuh `Authorization: Bearer <API_KEY>`)

3) Frontend
- `cd F:\chatbot\frontend`
- copy `F:\chatbot\frontend\.env.example` → `.env` lalu sesuaikan (`VITE_*`)
- `bun install`
- `bunx svelte-kit sync`
- `bun run dev`

## Dokumen penting
- `F:\chatbot\AGENTS.md`
- `F:\chatbot\docs\00_overview.md`
- `F:\chatbot\docs\roadmap.md`

