//! # OpenInference Semantic Conventions for Rust
//!
//! This crate provides attribute constants and types for the
//! [OpenInference semantic conventions](https://github.com/Arize-ai/openinference/blob/main/spec/semantic_conventions.md).
//!
//! OpenInference is a set of conventions for instrumenting LLM applications,
//! compatible with OpenTelemetry and designed to work with observability platforms
//! like [Arize Phoenix](https://phoenix.arize.com/).
//!
//! ## Usage
//!
//! ```rust
//! use openinference_semantic_conventions::{SpanKind, attributes};
//! use opentelemetry::KeyValue;
//!
//! // Set the span kind
//! let kind = KeyValue::new(attributes::OPENINFERENCE_SPAN_KIND, SpanKind::Llm.as_str());
//!
//! // Set LLM attributes
//! let model = KeyValue::new(attributes::llm::MODEL_NAME, "gpt-4");
//! let tokens = KeyValue::new(attributes::llm::token_count::TOTAL, 150i64);
//! ```
//!
//! ## OTel GenAI Compatibility
//!
//! This crate also provides aliases for [OpenTelemetry GenAI semantic conventions](https://opentelemetry.io/docs/specs/semconv/gen-ai/)
//! via the [`gen_ai`] module, enabling compatibility with both OpenInference and OTel backends.

pub mod attributes;
pub mod gen_ai;
mod span_kind;

pub use span_kind::SpanKind;

/// Re-export commonly used items
pub mod prelude {
    pub use crate::attributes;
    pub use crate::gen_ai;
    pub use crate::SpanKind;
}
