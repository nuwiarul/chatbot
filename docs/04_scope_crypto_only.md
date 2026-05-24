# Scope Policy: Crypto-only Assistant

Tujuan: chatbot ini **hanya** menjawab pertanyaan yang relevan dengan topik crypto.

## Apa maksud “crypto-only”?
Chatbot boleh menjawab jika pertanyaan berhubungan langsung dengan:
- cryptocurrency / blockchain / web3
- exchange, trading, market structure, tokenomics (umum)
- wallet, seed phrase, private key (konsep & best practices)
- DeFi, NFT, mining, L2, smart contract (konsep)
- regulasi/kebijakan yang *spesifik* terkait crypto

Chatbot **tidak** menjawab jika pertanyaan:
- jelas di luar domain (mis. resep masakan, matematika umum, sejarah umum)
- terlalu umum dan tidak ada kaitannya ke crypto

Respon yang diharapkan saat out-of-scope:
- Menolak singkat, lalu minta user rephrase/bertanya ulang dalam konteks crypto.

## Di mana policy ini diimplementasikan?
Saat ini ada di **system prompt** (Phase 1) di:
- `backend/src/http/llm.rs`

Keterbatasan system prompt:
- Ini “soft guardrail”: model biasanya patuh, tapi bisa saja gagal pada prompt injection.

## Rencana penguatan (Phase 5)
Untuk enforcement yang lebih kuat, kita akan tambah:
1) **Pre-check di backend** (classifier sederhana / heuristic keyword / small model):
   - jika tidak crypto → backend balas 400/200 dengan pesan “out of scope” tanpa memanggil LLM.
2) **RAG grounding**:
   - jika crypto tapi retrieval tidak menemukan konteks yang cukup → jawab “tidak ada info yang didukung dokumen”.
3) Audit log untuk monitoring pertanyaan out-of-scope.

## Contoh

### In-scope
User: “Apa itu Bitcoin halving dan dampaknya ke supply?”
- Jawab: penjelasan halving + dampak supply, dsb.

User: “Gimana cara kerja wallet non-custodial?”
- Jawab: konsep seed phrase/private key + keamanan.

### Out-of-scope
User: “Buatkan saya CV”
- Jawab: “Saya hanya khusus topik crypto…”

User: “Jelaskan hukum Newton”
- Jawab: tolak, minta hubungkan ke crypto jika ada konteks.

