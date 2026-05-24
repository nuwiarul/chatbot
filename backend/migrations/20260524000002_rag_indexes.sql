-- Indexes for RAG

CREATE INDEX IF NOT EXISTS idx_chunks_document_id ON chunks(document_id);
CREATE INDEX IF NOT EXISTS idx_chunks_created_at ON chunks(created_at);

-- NOTE:
-- pgvector ANN indexes (ivfflat/hnsw) require a fixed vector dimension (e.g. vector(1536)).
-- In Phase 2 we keep `embeddings.embedding` dimensionless because the embedding model/dim
-- can change during experimentation. We'll add the ANN index after we lock the dimension.
