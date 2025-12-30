//! # OpenInference Instrumentation for Rust
//!
//! This crate provides instrumentation helpers for creating OpenInference-compliant
//! spans in Rust applications using the `tracing` crate.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use openinference_instrumentation::LlmSpanBuilder;
//! use openinference_semantic_conventions::SpanKind;
//!
//! // Create an LLM span with OpenInference attributes
//! let span = LlmSpanBuilder::new("gpt-4")
//!     .provider("openai")
//!     .temperature(0.7)
//!     .input_message(0, "user", "Hello, world!")
//!     .build();
//!
//! // Use the span with tracing
//! let _guard = span.enter();
//! // ... perform LLM call ...
//! ```

pub mod span_builder;

pub use span_builder::{
    ChainSpanBuilder, EmbeddingSpanBuilder, LlmSpanBuilder, RetrieverSpanBuilder, ToolSpanBuilder,
};

/// Re-export semantic conventions for convenience.
pub use openinference_semantic_conventions as semconv;
