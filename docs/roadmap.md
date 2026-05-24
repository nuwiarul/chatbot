# Roadmap (Belajar Bertahap)

Checklist ini dipakai supaya progres rapi dan mudah di-review.

## Phase 0 — Bootstrap
- [x] Struktur folder `infra/ docs/ backend/ frontend/`
- [x] Postgres + pgvector via `infra/docker-compose.yml`
- [x] Backend skeleton + `GET /health`
- [x] Frontend skeleton + halaman chat minimal

## Phase 1 — Chat end-to-end (tanpa RAG)
- [x] Backend `POST /v1/chat` → forward ke llama.cpp `/v1/chat/completions`
- [x] Backend auth API key untuk semua endpoint selain `/health` (middleware/router-level)
- [x] Frontend chat UI call backend `/v1/chat`
- [x] Docs: kontrak request/response chat + cara troubleshooting
- [x] Streaming chat (SSE) untuk LLM
- [x] Render Markdown assistant (code/quote) di UI
- [x] Frontend build sebagai SPA (no SSR)
- [x] System prompt: paksa output Markdown + fenced code blocks

## Phase 2 — Schema RAG (pgvector)
- [x] SQLx migrations: `documents`, `chunks`, `embeddings(vector)`
- [x] Index vektor (pgvector) untuk retrieval

## Phase 3 — Ingestion
- [ ] Endpoint ingest teks sederhana
- [ ] Chunking + overlap + dedup hash
- [ ] Embedding via llama.cpp `/v1/embeddings`

## Phase 4 — Retrieval + RAG prompt
- [ ] Endpoint retrieve: embed query → top-k chunk
- [ ] Chat with context + sources

## Phase 5 — Guardrails anti prompt-injection
- [ ] Treat retrieved text sebagai untrusted data (label/data block)
- [ ] Context filtering (heuristic) untuk pola injection
- [ ] “Answer grounding”: jika tidak ada evidence → bilang tidak ditemukan
- [ ] Audit log retrieval (query, chunk ids, score)
