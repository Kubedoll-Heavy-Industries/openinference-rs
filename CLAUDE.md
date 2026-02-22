# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

openinference-rs is the first Rust implementation of the [OpenInference](https://github.com/Arize-ai/openinference) semantic conventions for LLM observability. It provides typed attribute constants and span builders that emit OpenTelemetry-compatible traces consumable by Arize Phoenix, Grafana, Jaeger, Datadog, and any OTLP backend.

The upstream reference implementations live in Python and JavaScript at `Arize-ai/openinference`. A local clone at `../openinference/` can be used as a reference for the spec and for API parity checks.

This project is intended for publication to crates.io and is also consumed by the [geodesic](../geodesic/) workspace (a distributed LLM inference engine) as a `tower::Layer` middleware for axum-based chat completion endpoints.

## Build and Test

```bash
cargo build --all                    # build both crates
cargo build --all --all-features     # build with optional serde support
cargo test --all                     # run all tests (11 unit + doc-tests)
cargo test -p openinference-semantic-conventions  # test just semconv crate
cargo test -p openinference-instrumentation       # test just instrumentation crate
cargo test --all -- test_llm_span    # run a single test by name
cargo deny check                     # license + advisory + ban + source checks
cargo vet --locked                   # supply chain audit (cargo-vet)
cargo semver-checks check-release --all  # semver compatibility check
```

MSRV is 1.93. Edition 2021. Published on [crates.io](https://crates.io/crates/openinference-semantic-conventions).

## Workspace Structure

Two crates with a strict layering:

```
openinference-semantic-conventions/    # Crate 1: Pure constants, zero instrumentation deps
  src/attributes.rs                    # ~100 attribute key constants (llm.*, embedding.*, tool.*, etc.)
  src/span_kind.rs                     # SpanKind enum (9 variants: Llm, Embedding, Chain, Tool, Agent, ...)
  src/gen_ai.rs                        # OTel GenAI semconv constants + bidirectional mapping functions

openinference-instrumentation/         # Crate 2: Span builders on top of tracing + OTel
  src/span_builder.rs                  # LlmSpanBuilder, EmbeddingSpanBuilder, ChainSpanBuilder, etc.
  src/lib.rs                           # Re-exports, SpanConfig, helper functions
```

**Dependency direction**: instrumentation depends on semantic-conventions, never the reverse. The semconv crate's only runtime dep is `opentelemetry` (for `Key`/`Value` types). The instrumentation crate adds `tracing`, `tracing-opentelemetry`, and `serde_json`.

## Key Architectural Patterns

### Dual Attribute Emission

Every span builder emits both OpenInference (`llm.*`) and OTel GenAI (`gen_ai.*`) attributes simultaneously. This is controlled by `SpanConfig::emit_gen_ai_attributes` (default: true). The `gen_ai` module provides `map_openinference_to_gen_ai()` and `map_gen_ai_to_openinference()` for translation.

### Indexed Attribute Keys

OpenInference uses flat OTel attributes with dot-separated indices: `llm.input_messages.0.message.role`. Helper functions in `attributes.rs` generate these (e.g., `input_messages::role(index)`). These use `Box::leak()` to produce `&'static str` keys for `Key::from_static_str()` — this is intentional for per-request hot-path performance but means keys are never freed.

### Privacy Controls

`SpanConfig::record_content` (default: false) gates recording of prompt/completion text. This maps to the `OPENINFERENCE_HIDE_INPUTS` / `OPENINFERENCE_HIDE_OUTPUTS` environment variables from the [OpenInference Configuration spec](https://github.com/Arize-ai/openinference/blob/main/spec/configuration.md). The full spec defines 13 env vars for `TraceConfig` — only a subset is implemented so far.

### Builder Pattern

All span types use fluent builders that produce `tracing::Span` instances with pre-populated OTel attributes:

```rust
LlmSpanBuilder::new("model-name").provider("openai").temperature(0.7).build()
```

Post-creation helpers like `record_token_usage()`, `record_output_message()`, and `record_error()` fill in response-time fields.

## Pinned Dependency Versions

These versions must stay in sync with the geodesic workspace:

- `opentelemetry = "0.31"`
- `tracing-opentelemetry = "0.32"`
- `tracing = "0.1"`

## Spec References

- [OpenInference Semantic Conventions](https://arize-ai.github.io/openinference/spec/semantic_conventions.html) — full attribute inventory
- [OpenInference Configuration](https://github.com/Arize-ai/openinference/blob/main/spec/configuration.md) — the 13 TraceConfig env vars
- [OTel GenAI Semantic Conventions](https://opentelemetry.io/docs/specs/semconv/gen-ai/) — the competing standard we also support
- [Tracking issue](https://github.com/Kubedoll-Heavy-Industries/geodesic/issues/138) — phased roadmap and engineering context

## CI/CD

All workflows live in `.github/workflows/`:

- **ci.yml** — fmt, clippy, test matrix (MSRV 1.93 / stable / nightly × ubuntu + stable × macOS), docs, semver checks. Runs on push to main and PRs.
- **security.yml** — cargo-audit, cargo-deny (advisories/bans/licenses/sources), cargo-vet. Runs on push/PR and daily at 06:00 UTC.
- **pr.yml** — Enforces conventional commit PR titles via semantic-pull-request.
- **release.yml** — release-please creates version bump PRs on push to main. On merge, gates on CI + security, then publishes to crates.io via OIDC trusted publishing, generates provenance attestations, and uploads SBOMs.

Releases use [release-please](https://github.com/googleapis/release-please) with linked versions (both crates always release at the same version). Configuration in `release-please-config.json` and `.release-please-manifest.json`.
