# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-06-15

### Added

- `openinference-semantic-conventions` crate with ~100 attribute key constants matching the [OpenInference specification](https://github.com/Arize-ai/openinference/blob/main/spec/semantic_conventions.md)
- `SpanKind` enum with 9 variants: LLM, Embedding, Chain, Tool, Agent, Retriever, Reranker, Guardrail, Evaluator
- OTel GenAI semantic conventions (`gen_ai.*`) with bidirectional mapping to OpenInference attributes
- `openinference-instrumentation` crate with fluent span builders (`LlmSpanBuilder`, `EmbeddingSpanBuilder`, `ChainSpanBuilder`, etc.)
- `SpanConfig` for controlling dual attribute emission and privacy settings
- Post-creation helpers: `record_token_usage()`, `record_output_message()`, `record_error()`
- Privacy controls via `OPENINFERENCE_HIDE_INPUTS` / `OPENINFERENCE_HIDE_OUTPUTS` environment variables
- Optional `serde` feature for serialization support on semantic convention types
- Indexed attribute key generation (e.g., `llm.input_messages.0.message.role`)

[0.1.0]: https://github.com/cagyirey/openinference-rs/releases/tag/v0.1.0
