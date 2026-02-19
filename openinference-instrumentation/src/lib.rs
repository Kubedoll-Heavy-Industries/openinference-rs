//! # OpenInference Instrumentation for Rust
//!
//! This crate provides instrumentation helpers for creating OpenInference-compliant
//! spans in Rust applications using the `tracing` crate.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use openinference_instrumentation::{LlmSpanBuilder, TraceConfig};
//!
//! // Create an LLM span with OpenInference attributes
//! let config = TraceConfig::default();
//! let span = LlmSpanBuilder::new("gpt-4")
//!     .config(config)
//!     .provider("openai")
//!     .temperature(0.7)
//!     .input_message("user", "Hello, world!")
//!     .build();
//!
//! // Use the span with tracing
//! let _guard = span.enter();
//! // ... perform LLM call ...
//! ```

pub mod config;
pub mod span_builder;

pub use config::{TraceConfig, TraceConfigBuilder, REDACTED};
pub use span_builder::{
    AgentSpanBuilder, ChainSpanBuilder, Document, EmbeddingSpanBuilder, EvaluatorSpanBuilder,
    GuardrailSpanBuilder, LlmSpanBuilder, RerankerSpanBuilder, RetrieverSpanBuilder,
    ToolSpanBuilder,
};
pub use span_builder::{
    record_error, record_output_message, record_output_tool_call, record_output_value,
    record_reranker_output_documents, record_retrieval_documents, record_token_usage,
};

/// Re-export semantic conventions for convenience.
pub use openinference_semantic_conventions as semconv;
