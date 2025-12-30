# openinference-rs

Rust implementation of [OpenInference](https://github.com/Arize-ai/openinference) semantic conventions for LLM observability.

[![Crates.io](https://img.shields.io/crates/v/openinference-semantic-conventions.svg)](https://crates.io/crates/openinference-semantic-conventions)
[![Documentation](https://docs.rs/openinference-semantic-conventions/badge.svg)](https://docs.rs/openinference-semantic-conventions)
[![License](https://img.shields.io/crates/l/openinference-semantic-conventions.svg)](LICENSE)

## Overview

OpenInference is a set of conventions for instrumenting LLM applications, compatible with [OpenTelemetry](https://opentelemetry.io/) and designed to work with observability platforms like [Arize Phoenix](https://phoenix.arize.com/).

This repository provides Rust crates for:

- **Semantic conventions** - Attribute constants matching the [OpenInference specification](https://github.com/Arize-ai/openinference/blob/main/spec/semantic_conventions.md)
- **OTel GenAI compatibility** - Aliases for [OpenTelemetry GenAI semantic conventions](https://opentelemetry.io/docs/specs/semconv/gen-ai/)
- **Instrumentation helpers** - Span builders for easy integration with the `tracing` crate

## Crates

| Crate | Description |
|-------|-------------|
| [`openinference-semantic-conventions`](./openinference-semantic-conventions) | Attribute constants and types |
| [`openinference-instrumentation`](./openinference-instrumentation) | Span builders and helpers |

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
openinference-semantic-conventions = "0.1"
openinference-instrumentation = "0.1"
```

### Basic Usage

```rust
use openinference_semantic_conventions::{attributes, SpanKind};
use openinference_instrumentation::LlmSpanBuilder;

// Create an LLM span with OpenInference attributes
let span = LlmSpanBuilder::new("gpt-4")
    .provider("openai")
    .temperature(0.7)
    .max_tokens(1000)
    .build();

let _guard = span.enter();
// ... perform LLM call ...
```

### Using Attribute Constants Directly

```rust
use openinference_semantic_conventions::{attributes, gen_ai, SpanKind};
use opentelemetry::KeyValue;

// OpenInference attributes
let kind = KeyValue::new(attributes::OPENINFERENCE_SPAN_KIND, SpanKind::Llm.as_str());
let model = KeyValue::new(attributes::llm::MODEL_NAME, "gpt-4");
let tokens = KeyValue::new(attributes::llm::token_count::TOTAL, 150i64);

// OTel GenAI attributes (for dual compatibility)
let gen_ai_model = KeyValue::new(gen_ai::request::MODEL, "gpt-4");
let input_tokens = KeyValue::new(gen_ai::usage::INPUT_TOKENS, 100i64);
```

## Span Kinds

OpenInference defines the following span kinds:

| Kind | Description |
|------|-------------|
| `LLM` | Call to a Large Language Model |
| `EMBEDDING` | Call to generate embeddings |
| `CHAIN` | Workflow/pipeline step or glue code |
| `TOOL` | External tool/function execution |
| `AGENT` | Reasoning block using LLMs and tools |
| `RETRIEVER` | Vector store or database query |
| `RERANKER` | Document reranking |
| `GUARDRAIL` | Input/output validation |
| `EVALUATOR` | Model output evaluation |

## Dual Attribute Support

This library supports both OpenInference (`llm.*`) and OTel GenAI (`gen_ai.*`) attribute conventions for maximum compatibility:

```rust
use openinference_semantic_conventions::gen_ai;

// Map between conventions
let otel_key = gen_ai::map_openinference_to_gen_ai("llm.model_name");
let oi_key = gen_ai::map_gen_ai_to_openinference("gen_ai.request.model");
```

## Integration with Observability Backends

Spans created with this library are compatible with:

- [Arize Phoenix](https://phoenix.arize.com/) (native OpenInference support)
- [Grafana Tempo](https://grafana.com/oss/tempo/) (via OTel)
- [Jaeger](https://www.jaegertracing.io/) (via OTel)
- [Datadog](https://www.datadoghq.com/) (OTel GenAI support)
- [Honeycomb](https://www.honeycomb.io/) (via OTel)
- Any OpenTelemetry-compatible backend

## Configuration

### Environment Variables

```bash
# Privacy settings (opt-in content recording)
OPENINFERENCE_HIDE_INPUTS=false
OPENINFERENCE_HIDE_OUTPUTS=false

# OTel exporter configuration
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
OTEL_SERVICE_NAME=my-llm-app
```

## License

Apache-2.0

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## Related Projects

- [OpenInference](https://github.com/Arize-ai/openinference) - Original specification and Python/JS implementations
- [Arize Phoenix](https://github.com/Arize-ai/phoenix) - AI observability platform
- [OpenTelemetry Rust](https://github.com/open-telemetry/opentelemetry-rust) - Rust OTel SDK
