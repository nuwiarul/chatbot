# RAG (Retrieval-Augmented Generation) — Penjelasan Lengkap

Dokumen ini menjelaskan konsep RAG dari nol, kenapa kita butuh vector database, dan bagaimana alur data pada project ini (SvelteKit + Axum + Postgres/pgvector + llama.cpp).

## 1) Masalah yang diselesaikan RAG
LLM “murni chat” punya keterbatasan:
- **Tidak tahu data privat kamu** (dokumen internal, SOP, catatan proyek).
- **Pengetahuan bisa outdated** (tergantung model).
- Kalau dipaksa jawab tentang sesuatu yang tidak dia ketahui, model bisa **halusinasi**.

RAG menambahkan kemampuan:
1) **Cari informasi relevan** dari dokumen kamu.
2) **Masukkan potongan informasi** itu ke prompt.
3) LLM menjawab dengan “berbasis konteks” (grounded), idealnya disertai sumber.

Jadi RAG = *Search + Prompting + LLM*.

## 2) Gambaran besar pipeline RAG
Ada 2 jalur utama:

### A. Jalur Ingestion (memasukkan dokumen ke “knowledge base”)
1. Ambil dokumen (teks/file).
2. Pecah dokumen jadi **chunks** (potongan kecil).
3. Buat **embedding** untuk setiap chunk.
4. Simpan chunk + embedding ke database.

### B. Jalur Question Answering (saat user bertanya)
1. User bertanya.
2. Pertanyaan di-embedding.
3. Cari chunk paling mirip (top-k) dengan similarity search.
4. Susun prompt: system policy + user question + “context chunks”.
5. LLM menjawab.

## 3) Apa itu embedding?
Embedding adalah representasi angka (vektor) dari teks.
- Teks → model embedding → vektor `[x1, x2, ... xN]`
- Teks yang “mirip makna” akan punya vektor yang posisinya “dekat”.

Contoh intuitif (bukan angka asli):
- “cara reset password” dekat dengan “lupa kata sandi”
- “cara masak nasi goreng” jauh dari “cara reset password”

Dengan embedding, kita bisa melakukan pencarian semantik, bukan sekadar keyword.

## 4) Kenapa butuh vector database / pgvector?
Kalau kita simpan embedding di DB, kita butuh query “cari yang paling dekat”.
Itu disebut **vector similarity search**.

Kita pakai Postgres + pgvector karena:
- Data relasional (dokumen, metadata) dan data vektor (embedding) bisa di satu tempat.
- Cocok untuk tahap belajar dan cukup production untuk banyak use case.

## 5) Kenapa dokumen harus di-chunk?
Alasan utama:
- LLM punya batas konteks (mis. 4096 token).
- Embedding dan retrieval lebih akurat kalau potongan teks tidak terlalu panjang.
- Saat retrieval, kita hanya ambil potongan yang relevan, bukan 1 dokumen besar.

Chunking yang baik biasanya:
- Ukuran cukup untuk memuat satu ide utuh.
- Ada overlap kecil supaya informasi tidak “kepotong” tepat di batas chunk.

## 6) Bentuk data di project ini (Phase 2)
Di backend kita sudah siapkan schema:

### `documents`
Satu “dokumen” = satu sumber.
Contoh:
- `source="local:file"` title `"SOP Onboarding"`
- metadata: `{ "path": "docs/sop.md" }`

### `chunks`
Satu dokumen dipecah jadi banyak chunks.
Kolom penting:
- `document_id` (relasi ke documents)
- `chunk_index` urutan potongan
- `content` isi chunk
- `content_hash` untuk dedup (mis. sha256)

### `embeddings`
Satu chunk punya satu embedding.
Kolom penting:
- `chunk_id` (relasi ke chunks)
- `embedding` tipe `vector` (pgvector)
- `model` nama model embedding yang dipakai

## 6.1) Cara cek Postgres support `vector` dan `pgcrypto`
Di project ini, extension dipakai untuk:
- `vector` (pgvector): menyimpan embedding dan melakukan similarity search.
- `pgcrypto`: helper untuk UUID random (mis. `gen_random_uuid()`), hashing, dll.

### A) Cek via SQL (psql / DBeaver / TablePlus)
Jalankan query ini:

```sql
-- Apakah extension tersedia/terpasang?
SELECT extname, extversion
FROM pg_extension
WHERE extname IN ('vector', 'pgcrypto')
ORDER BY extname;
```

Kalau sudah aktif, hasilnya akan mengembalikan 2 baris: `vector` dan `pgcrypto` beserta versinya.

Jika tidak ada, kamu bisa aktifkan (butuh hak yang cukup) dengan:
```sql
CREATE EXTENSION IF NOT EXISTS vector;
CREATE EXTENSION IF NOT EXISTS pgcrypto;
```

### B) Cek tipe `vector` ada atau tidak
Ini cara lain untuk memastikan tipe datanya tersedia:

```sql
SELECT 1
FROM pg_type
WHERE typname = 'vector';
```

Kalau mengembalikan 1 baris, artinya type `vector` ada.

### C) Cek fungsi `gen_random_uuid()` dari `pgcrypto`

```sql
SELECT gen_random_uuid();
```

Kalau berhasil dan mengembalikan UUID, `pgcrypto` aktif.

### Catatan (relevan ke migrasi kita)
Pada migration Phase 2 kita memakai:
- `gen_random_uuid()` sebagai default primary key UUID
- kolom `embedding vector`

Jadi kalau extension belum aktif, migrasi bisa gagal.

## 7) Bagaimana retrieval bekerja (inti RAG)
Misal user bertanya:
> “Bagaimana cara menambahkan user baru di sistem kita?”

Langkah:
1) Backend memanggil `/v1/embeddings` ke llama.cpp untuk pertanyaan itu → dapat `query_embedding`.
2) Backend query DB: cari embedding chunk yang paling dekat dengan `query_embedding`.
3) Ambil `top_k` chunks (mis. 5) sebagai konteks.

LLM kemudian mendapat prompt seperti:
- System: kebijakan + format
- User: pertanyaan user
- Context (DATA, bukan instruksi): chunk1..chunkK

LLM menjawab berdasarkan konteks.

## 8) Contoh RAG end-to-end (contoh sederhana)
Bayangkan kamu punya dokumen:

Judul: “Panduan Refund”
Isi (teks):
> Refund bisa diproses maksimal 7 hari kerja setelah barang diterima.  
> Syarat: barang lengkap, nomor invoice tersedia.

### Ingestion
Dokumen di-chunk mis. jadi 1 chunk:
- chunk0: “Refund bisa diproses maksimal 7 hari kerja ... invoice tersedia.”

Lalu di-embedding dan disimpan.

### Tanya jawab
User bertanya:
> “Berapa lama refund diproses?”

Retrieval menemukan chunk refund sebagai paling mirip.
LLM diberi konteks, lalu jawab:
> “Refund diproses maksimal 7 hari kerja setelah barang diterima.”
Dan bisa menampilkan sources (mis. `document_id`, `chunk_id`).

## 9) “Vector DB bagus, tapi bukan satu-satunya”
Alternatif selain pgvector:
- Qdrant/Weaviate/Milvus: vector DB khusus (fitur vektor lebih lengkap).
- Elasticsearch/OpenSearch vector: kalau sudah pakai stack search.
- SQLite vector extension: ringan untuk local learning.

Untuk belajar + integrasi SQLx, pgvector itu pilihan paling mudah.

## 10) RAG bukan magic: kualitas ditentukan oleh 4 hal
1) **Kualitas dokumen** (jelas, konsisten).
2) **Chunking** (ukuran + overlap + pemisah).
3) **Embedding model** (cocok untuk bahasa kamu, domain kamu).
4) **Prompt assembly** (cara menyajikan konteks ke LLM).

## 11) RAG dan prompt injection (penting)
Saat pakai RAG, kamu mengambil teks dari dokumen. Teks dokumen bisa mengandung instruksi “jahat”.
Prinsip aman:
- Teks retrieved harus dianggap **DATA / untrusted**, bukan instruksi.
- System/developer policy harus selalu menang atas isi dokumen.

Nanti di Phase 5 kita buat guardrails yang lebih kuat.

## 12) Next: apa yang kita kerjakan di Phase 3?
Phase 3 = “Ingestion pipeline”:
- endpoint ingest teks
- chunking + hash dedup
- panggil embedding endpoint `/v1/embeddings`
- simpan ke `documents/chunks/embeddings`

Setelah itu Phase 4 = retrieval top-k + RAG prompt.
