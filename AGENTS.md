# AGENTS.md (Repo Guidelines)

## Tujuan
Repo ini adalah project belajar chatbot + RAG dengan stack:
- Frontend: SvelteKit 5 + Tailwind CSS 4 + Vite, package manager: Bun
- Backend: Rust + Axum 0.8 + SQLx
- Infra: docker-compose + konfigurasi service (Postgres + pgvector, dll)
- Docs: dokumentasi arsitektur, ADR, catatan belajar

## Struktur folder utama
- `infra/`    -> docker-compose, config service, init scripts
- `docs/`     -> dokumentasi & catatan (Markdown)
- `backend/`  -> service Rust (Axum + SQLx + RAG)
- `frontend/` -> app SvelteKit

Folder tambahan boleh dibuat jika jelas fungsinya (mis. `scripts/`, `assets/`, `tools/`).

## Struktur folder (proposal awal)
Struktur ini adalah baseline untuk meminimalkan duplicate code dan memaksa pemisahan concern.

```txt
repo-root/
  AGENTS.md
  infra/
    docker-compose.yml
    postgres/
      init/
        001_extensions.sql
        002_schema.sql
      conf/
        postgresql.conf
  docs/
    00_overview.md
    01_architecture.md
    02_rag_pipeline.md
    03_guardrails.md
    adr/
      0001_vector_store_pgvector.md
  backend/
    Cargo.toml
    Cargo.lock
    .env.example
    migrations/
    src/
      main.rs
      lib.rs
      config.rs
      error.rs
      http/
        mod.rs
        routes.rs
      handlers/
        mod.rs
        health.rs
        chat.rs
        rag_ingest.rs
        rag_retrieve.rs
      services/
        mod.rs
        llama_client.rs
        embeddings.rs
        chunking.rs
        rag.rs
      db/
        mod.rs
        models.rs
        queries.rs
  frontend/
    package.json
    bun.lock
    .env.example
    svelte.config.js
    vite.config.ts
    src/
      app.css
      lib/
        components/
          chat/
            ChatPane.svelte
            MessageList.svelte
            MessageInput.svelte
          ui/
            Button.svelte
            Textarea.svelte
        api/
          client.ts
          chat.ts
          rag.ts
        stores/
          chat.ts
        utils/
          env.ts
          types.ts
      routes/
        +layout.svelte
        +page.svelte
```

## Aturan kontribusi (penting)
1. Jangan implement fitur besar tanpa diskusi/plan singkat di `docs/` (minimal 1 dokumen ringkas).
2. Hindari duplicate code:
   - Frontend: utamakan komponen reusable + util bersama.
   - Backend: utamakan module/service reusable; hindari copy-paste handler.
   - Frontend: TypeScript wajib; **dilarang** menggunakan `any`. Struktur data harus jelas (types/interfaces/zod schema bila diperlukan).
3. Konfigurasi harus terdokumentasi:
   - Setiap env var yang dipakai harus ada di `.env.example` (di root atau per folder).
4. Security baseline:
   - Jangan commit secret/API key.
   - Semua endpoint selain `/health` harus siap diberi auth (minimal API key) ketika sudah masuk fase integrasi.
   - Data RAG (retrieved chunks) diperlakukan sebagai **untrusted**.
5. Testing baseline:
   - Backend: unit test untuk chunking + integration test untuk query DB (minimal).
   - Frontend: minimal test untuk util penting bila sudah ada test runner.
6. Formatting/Linting:
   - Backend: `cargo fmt` dan `cargo clippy` (jika dipakai) harus clean.
   - Frontend: gunakan formatter/linter yang disepakati di `frontend/` (jangan format manual beda-beda).

## Konvensi umum
- Nama paket/module jelas dan konsisten.
- Error handling eksplisit (backend), jangan `unwrap()` di code production path.
- Logging terstruktur (backend), jangan log data sensitif.
