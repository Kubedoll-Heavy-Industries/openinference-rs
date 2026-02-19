//! Span builders for OpenInference-compliant instrumentation.
//!
//! These builders provide a fluent API for creating spans with the correct
//! OpenInference attributes, and optionally dual-writing OTel GenAI attributes.
//! All attributes are set via `OpenTelemetrySpanExt::set_attribute()` so that
//! dynamic, indexed keys (e.g. `llm.input_messages.0.message.role`) work correctly.

use crate::config::{TraceConfig, REDACTED};
use openinference_semantic_conventions::attributes;
use openinference_semantic_conventions::gen_ai;
use openinference_semantic_conventions::SpanKind;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

// =============================================================================
// LLM Span Builder
// =============================================================================

/// Builder for LLM (Large Language Model) spans.
///
/// # Example
///
/// ```rust,ignore
/// use openinference_instrumentation::{LlmSpanBuilder, TraceConfig};
///
/// let config = TraceConfig::default();
/// let span = LlmSpanBuilder::new("gpt-4")
///     .config(config)
///     .provider("openai")
///     .temperature(0.7)
///     .max_tokens(1000)
///     .input_message("system", "You are a helpful assistant.")
///     .input_message("user", "Hello!")
///     .build();
/// ```
#[derive(Debug)]
pub struct LlmSpanBuilder {
    model_name: String,
    provider: Option<String>,
    system: Option<String>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    top_k: Option<i64>,
    max_tokens: Option<i64>,
    frequency_penalty: Option<f64>,
    presence_penalty: Option<f64>,
    input_messages: Vec<(String, String)>, // (role, content)
    invocation_parameters: Option<String>,
    input_value: Option<String>,
    output_value: Option<String>,
    tools: Vec<String>, // JSON schema strings
    config: TraceConfig,
}

impl LlmSpanBuilder {
    /// Create a new LLM span builder with the given model name.
    pub fn new(model_name: impl Into<String>) -> Self {
        Self {
            model_name: model_name.into(),
            provider: None,
            system: None,
            temperature: None,
            top_p: None,
            top_k: None,
            max_tokens: None,
            frequency_penalty: None,
            presence_penalty: None,
            input_messages: Vec::new(),
            invocation_parameters: None,
            input_value: None,
            output_value: None,
            tools: Vec::new(),
            config: TraceConfig::default(),
        }
    }

    /// Set the configuration for this builder.
    pub fn config(mut self, config: TraceConfig) -> Self {
        self.config = config;
        self
    }

    /// Set the LLM provider (e.g., "openai", "anthropic", "mistral.rs").
    pub fn provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = Some(provider.into());
        self
    }

    /// Set the LLM system (e.g., "openai", "anthropic").
    pub fn system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    /// Set the temperature parameter.
    pub fn temperature(mut self, temp: f64) -> Self {
        self.temperature = Some(temp);
        self
    }

    /// Set the top_p (nucleus sampling) parameter.
    pub fn top_p(mut self, top_p: f64) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Set the top_k parameter.
    pub fn top_k(mut self, top_k: i64) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Set the maximum tokens to generate.
    pub fn max_tokens(mut self, max_tokens: i64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Set the frequency penalty.
    pub fn frequency_penalty(mut self, penalty: f64) -> Self {
        self.frequency_penalty = Some(penalty);
        self
    }

    /// Set the presence penalty.
    pub fn presence_penalty(mut self, penalty: f64) -> Self {
        self.presence_penalty = Some(penalty);
        self
    }

    /// Add an input message. Messages are indexed in the order they are added.
    ///
    /// Content is subject to `TraceConfig` privacy controls.
    pub fn input_message(mut self, role: impl Into<String>, content: impl Into<String>) -> Self {
        self.input_messages.push((role.into(), content.into()));
        self
    }

    /// Set the invocation parameters as a JSON string.
    pub fn invocation_parameters(mut self, params: impl Into<String>) -> Self {
        self.invocation_parameters = Some(params.into());
        self
    }

    /// Set the input value (e.g., the raw prompt or request body).
    pub fn input_value(mut self, value: impl Into<String>) -> Self {
        self.input_value = Some(value.into());
        self
    }

    /// Set the output value (e.g., the raw response body).
    pub fn output_value(mut self, value: impl Into<String>) -> Self {
        self.output_value = Some(value.into());
        self
    }

    /// Add a tool schema (JSON string) available to the LLM.
    pub fn tool(mut self, json_schema: impl Into<String>) -> Self {
        self.tools.push(json_schema.into());
        self
    }

    /// Build the span.
    ///
    /// Returns a `tracing::Span` with all the configured attributes set via
    /// `OpenTelemetrySpanExt::set_attribute()`.
    pub fn build(self) -> Span {
        let span_name = format!("llm {}", self.model_name);

        let span = tracing::info_span!("llm", otel.name = %span_name);

        // -- Core attributes --
        span.set_attribute(attributes::OPENINFERENCE_SPAN_KIND, SpanKind::Llm.as_str());
        span.set_attribute(attributes::llm::MODEL_NAME, self.model_name.clone());

        if let Some(ref provider) = self.provider {
            span.set_attribute(attributes::llm::PROVIDER, provider.clone());
        }
        if let Some(ref system) = self.system {
            span.set_attribute(attributes::llm::SYSTEM, system.clone());
        }

        // -- Invocation parameters --
        if let Some(ref params) = self.invocation_parameters {
            if !self.config.hide_llm_invocation_parameters {
                span.set_attribute(attributes::llm::INVOCATION_PARAMETERS, params.clone());
            } else {
                span.set_attribute(attributes::llm::INVOCATION_PARAMETERS, REDACTED);
            }
        }

        // -- Input value --
        if let Some(ref input) = self.input_value {
            if !self.config.hide_inputs {
                span.set_attribute(attributes::input::VALUE, input.clone());
            } else {
                span.set_attribute(attributes::input::VALUE, REDACTED);
            }
        }

        // -- Output value --
        if let Some(ref output) = self.output_value {
            if !self.config.hide_outputs {
                span.set_attribute(attributes::output::VALUE, output.clone());
            } else {
                span.set_attribute(attributes::output::VALUE, REDACTED);
            }
        }

        // -- Input messages --
        if !self.input_messages.is_empty() {
            let hide_messages = self.config.should_hide_input_messages();
            let hide_text = self.config.should_hide_input_text();

            for (i, (role, content)) in self.input_messages.iter().enumerate() {
                if hide_messages {
                    span.set_attribute(attributes::llm::input_messages::role(i), REDACTED);
                    span.set_attribute(attributes::llm::input_messages::content(i), REDACTED);
                } else {
                    span.set_attribute(attributes::llm::input_messages::role(i), role.clone());
                    if hide_text {
                        span.set_attribute(attributes::llm::input_messages::content(i), REDACTED);
                    } else {
                        span.set_attribute(attributes::llm::input_messages::content(i), content.clone());
                    }
                }
            }
        }

        // -- Tools --
        for (i, schema) in self.tools.iter().enumerate() {
            span.set_attribute(attributes::llm::tools::json_schema(i), schema.clone());
        }

        // -- OTel GenAI attributes --
        if self.config.emit_gen_ai_attributes {
            span.set_attribute(gen_ai::request::MODEL, self.model_name.clone());
            if let Some(ref provider) = self.provider {
                span.set_attribute(gen_ai::PROVIDER_NAME, provider.clone());
            }
            if let Some(ref system) = self.system {
                span.set_attribute(gen_ai::SYSTEM, system.clone());
            }
            if let Some(temp) = self.temperature {
                span.set_attribute(gen_ai::request::TEMPERATURE, temp);
            }
            if let Some(top_p) = self.top_p {
                span.set_attribute(gen_ai::request::TOP_P, top_p);
            }
            if let Some(top_k) = self.top_k {
                span.set_attribute(gen_ai::request::TOP_K, top_k);
            }
            if let Some(max_tokens) = self.max_tokens {
                span.set_attribute(gen_ai::request::MAX_TOKENS, max_tokens);
            }
            if let Some(freq) = self.frequency_penalty {
                span.set_attribute(gen_ai::request::FREQUENCY_PENALTY, freq);
            }
            if let Some(pres) = self.presence_penalty {
                span.set_attribute(gen_ai::request::PRESENCE_PENALTY, pres);
            }
        }

        span
    }
}

// =============================================================================
// Embedding Span Builder
// =============================================================================

/// Builder for embedding spans.
#[derive(Debug)]
pub struct EmbeddingSpanBuilder {
    model_name: String,
    texts: Vec<String>,
    input_value: Option<String>,
    config: TraceConfig,
}

impl EmbeddingSpanBuilder {
    /// Create a new embedding span builder with the given model name.
    pub fn new(model_name: impl Into<String>) -> Self {
        Self {
            model_name: model_name.into(),
            texts: Vec::new(),
            input_value: None,
            config: TraceConfig::default(),
        }
    }

    /// Set the configuration for this builder.
    pub fn config(mut self, config: TraceConfig) -> Self {
        self.config = config;
        self
    }

    /// Add a text to embed.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.texts.push(text.into());
        self
    }

    /// Add multiple texts to embed.
    pub fn texts(mut self, texts: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.texts.extend(texts.into_iter().map(Into::into));
        self
    }

    /// Set the input value.
    pub fn input_value(mut self, value: impl Into<String>) -> Self {
        self.input_value = Some(value.into());
        self
    }

    /// Build the span.
    pub fn build(self) -> Span {
        let span_name = format!("embedding {}", self.model_name);

        let span = tracing::info_span!("embedding", otel.name = %span_name);

        span.set_attribute(attributes::OPENINFERENCE_SPAN_KIND, SpanKind::Embedding.as_str());
        span.set_attribute(attributes::embedding::MODEL_NAME, self.model_name.clone());

        // Embedding texts
        let hide_text = self.config.hide_embeddings_text;
        for (i, text) in self.texts.iter().enumerate() {
            if hide_text {
                span.set_attribute(attributes::embedding::embeddings::text(i), REDACTED);
            } else {
                span.set_attribute(attributes::embedding::embeddings::text(i), text.clone());
            }
        }

        // Input value
        if let Some(ref input) = self.input_value {
            if !self.config.hide_inputs {
                span.set_attribute(attributes::input::VALUE, input.clone());
            } else {
                span.set_attribute(attributes::input::VALUE, REDACTED);
            }
        }

        span
    }
}

// =============================================================================
// Chain Span Builder
// =============================================================================

/// Builder for chain spans (workflow/pipeline steps).
#[derive(Debug)]
pub struct ChainSpanBuilder {
    name: String,
    input_value: Option<String>,
    input_mime_type: Option<String>,
    output_value: Option<String>,
    output_mime_type: Option<String>,
    config: TraceConfig,
}

impl ChainSpanBuilder {
    /// Create a new chain span builder with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            input_value: None,
            input_mime_type: None,
            output_value: None,
            output_mime_type: None,
            config: TraceConfig::default(),
        }
    }

    /// Set the configuration for this builder.
    pub fn config(mut self, config: TraceConfig) -> Self {
        self.config = config;
        self
    }

    /// Set the input value.
    pub fn input(mut self, value: impl Into<String>) -> Self {
        self.input_value = Some(value.into());
        self
    }

    /// Set the input MIME type.
    pub fn input_mime_type(mut self, mime_type: impl Into<String>) -> Self {
        self.input_mime_type = Some(mime_type.into());
        self
    }

    /// Set the output value.
    pub fn output(mut self, value: impl Into<String>) -> Self {
        self.output_value = Some(value.into());
        self
    }

    /// Set the output MIME type.
    pub fn output_mime_type(mut self, mime_type: impl Into<String>) -> Self {
        self.output_mime_type = Some(mime_type.into());
        self
    }

    /// Build the span.
    pub fn build(self) -> Span {
        let span = tracing::info_span!("chain", otel.name = %self.name);

        span.set_attribute(attributes::OPENINFERENCE_SPAN_KIND, SpanKind::Chain.as_str());

        if let Some(ref input) = self.input_value {
            if !self.config.hide_inputs {
                span.set_attribute(attributes::input::VALUE, input.clone());
            } else {
                span.set_attribute(attributes::input::VALUE, REDACTED);
            }
        }
        if let Some(ref mime_type) = self.input_mime_type {
            span.set_attribute(attributes::input::MIME_TYPE, mime_type.clone());
        }

        if let Some(ref output) = self.output_value {
            if !self.config.hide_outputs {
                span.set_attribute(attributes::output::VALUE, output.clone());
            } else {
                span.set_attribute(attributes::output::VALUE, REDACTED);
            }
        }
        if let Some(ref mime_type) = self.output_mime_type {
            span.set_attribute(attributes::output::MIME_TYPE, mime_type.clone());
        }

        span
    }
}

// =============================================================================
// Tool Span Builder
// =============================================================================

/// Builder for tool spans.
#[derive(Debug)]
pub struct ToolSpanBuilder {
    name: String,
    description: Option<String>,
    parameters: Option<String>,
    input_value: Option<String>,
    output_value: Option<String>,
    config: TraceConfig,
}

impl ToolSpanBuilder {
    /// Create a new tool span builder with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            parameters: None,
            input_value: None,
            output_value: None,
            config: TraceConfig::default(),
        }
    }

    /// Set the configuration for this builder.
    pub fn config(mut self, config: TraceConfig) -> Self {
        self.config = config;
        self
    }

    /// Set the tool description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the tool parameters (JSON string).
    pub fn parameters(mut self, parameters: impl Into<String>) -> Self {
        self.parameters = Some(parameters.into());
        self
    }

    /// Set the input value.
    pub fn input_value(mut self, value: impl Into<String>) -> Self {
        self.input_value = Some(value.into());
        self
    }

    /// Set the output value.
    pub fn output_value(mut self, value: impl Into<String>) -> Self {
        self.output_value = Some(value.into());
        self
    }

    /// Build the span.
    pub fn build(self) -> Span {
        let span_name = format!("tool {}", self.name);

        let span = tracing::info_span!("tool", otel.name = %span_name);

        span.set_attribute(attributes::OPENINFERENCE_SPAN_KIND, SpanKind::Tool.as_str());
        span.set_attribute(attributes::tool::NAME, self.name.clone());

        if let Some(ref desc) = self.description {
            span.set_attribute(attributes::tool::DESCRIPTION, desc.clone());
        }
        if let Some(ref params) = self.parameters {
            span.set_attribute(attributes::tool::PARAMETERS, params.clone());
        }

        if let Some(ref input) = self.input_value {
            if !self.config.hide_inputs {
                span.set_attribute(attributes::input::VALUE, input.clone());
            } else {
                span.set_attribute(attributes::input::VALUE, REDACTED);
            }
        }
        if let Some(ref output) = self.output_value {
            if !self.config.hide_outputs {
                span.set_attribute(attributes::output::VALUE, output.clone());
            } else {
                span.set_attribute(attributes::output::VALUE, REDACTED);
            }
        }

        span
    }
}

// =============================================================================
// Retriever Span Builder
// =============================================================================

/// Builder for retriever spans.
#[derive(Debug)]
pub struct RetrieverSpanBuilder {
    name: String,
    query: Option<String>,
    top_k: Option<i64>,
    config: TraceConfig,
}

impl RetrieverSpanBuilder {
    /// Create a new retriever span builder with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            query: None,
            top_k: None,
            config: TraceConfig::default(),
        }
    }

    /// Set the configuration for this builder.
    pub fn config(mut self, config: TraceConfig) -> Self {
        self.config = config;
        self
    }

    /// Set the retrieval query.
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// Set the top_k parameter.
    pub fn top_k(mut self, top_k: i64) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Build the span.
    pub fn build(self) -> Span {
        let span_name = format!("retriever {}", self.name);

        let span = tracing::info_span!("retriever", otel.name = %span_name);

        span.set_attribute(attributes::OPENINFERENCE_SPAN_KIND, SpanKind::Retriever.as_str());

        if let Some(ref query) = self.query {
            if !self.config.hide_inputs {
                span.set_attribute(attributes::input::VALUE, query.clone());
            } else {
                span.set_attribute(attributes::input::VALUE, REDACTED);
            }
        }

        span
    }
}

// =============================================================================
// Agent Span Builder
// =============================================================================

/// Builder for agent spans (autonomous reasoning blocks with LLM + tool use).
#[derive(Debug)]
pub struct AgentSpanBuilder {
    name: String,
    input_value: Option<String>,
    output_value: Option<String>,
    config: TraceConfig,
}

impl AgentSpanBuilder {
    /// Create a new agent span builder with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            input_value: None,
            output_value: None,
            config: TraceConfig::default(),
        }
    }

    /// Set the configuration for this builder.
    pub fn config(mut self, config: TraceConfig) -> Self {
        self.config = config;
        self
    }

    /// Set the input value.
    pub fn input_value(mut self, value: impl Into<String>) -> Self {
        self.input_value = Some(value.into());
        self
    }

    /// Set the output value.
    pub fn output_value(mut self, value: impl Into<String>) -> Self {
        self.output_value = Some(value.into());
        self
    }

    /// Build the span.
    pub fn build(self) -> Span {
        let span_name = format!("agent {}", self.name);

        let span = tracing::info_span!("agent", otel.name = %span_name);

        span.set_attribute(attributes::OPENINFERENCE_SPAN_KIND, SpanKind::Agent.as_str());
        span.set_attribute(attributes::agent::NAME, self.name.clone());

        if let Some(ref input) = self.input_value {
            if !self.config.hide_inputs {
                span.set_attribute(attributes::input::VALUE, input.clone());
            } else {
                span.set_attribute(attributes::input::VALUE, REDACTED);
            }
        }
        if let Some(ref output) = self.output_value {
            if !self.config.hide_outputs {
                span.set_attribute(attributes::output::VALUE, output.clone());
            } else {
                span.set_attribute(attributes::output::VALUE, REDACTED);
            }
        }

        span
    }
}

// =============================================================================
// Reranker Span Builder
// =============================================================================

/// A document for reranker/retriever input/output.
#[derive(Debug, Clone)]
pub struct Document {
    pub id: Option<String>,
    pub content: String,
    pub score: Option<f64>,
}

/// Builder for reranker spans.
#[derive(Debug)]
pub struct RerankerSpanBuilder {
    model_name: String,
    query: Option<String>,
    top_k: Option<i64>,
    input_documents: Vec<Document>,
    config: TraceConfig,
}

impl RerankerSpanBuilder {
    /// Create a new reranker span builder with the given model name.
    pub fn new(model_name: impl Into<String>) -> Self {
        Self {
            model_name: model_name.into(),
            query: None,
            top_k: None,
            input_documents: Vec::new(),
            config: TraceConfig::default(),
        }
    }

    /// Set the configuration for this builder.
    pub fn config(mut self, config: TraceConfig) -> Self {
        self.config = config;
        self
    }

    /// Set the reranking query.
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// Set the top_k parameter.
    pub fn top_k(mut self, top_k: i64) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Add an input document.
    pub fn input_document(mut self, doc: Document) -> Self {
        self.input_documents.push(doc);
        self
    }

    /// Build the span.
    pub fn build(self) -> Span {
        let span_name = format!("reranker {}", self.model_name);

        let span = tracing::info_span!("reranker", otel.name = %span_name);

        span.set_attribute(attributes::OPENINFERENCE_SPAN_KIND, SpanKind::Reranker.as_str());
        span.set_attribute(attributes::reranker::MODEL_NAME, self.model_name.clone());

        if let Some(ref query) = self.query {
            if !self.config.hide_inputs {
                span.set_attribute(attributes::reranker::QUERY, query.clone());
            } else {
                span.set_attribute(attributes::reranker::QUERY, REDACTED);
            }
        }

        if let Some(top_k) = self.top_k {
            span.set_attribute(attributes::reranker::TOP_K, top_k);
        }

        for (i, doc) in self.input_documents.iter().enumerate() {
            if let Some(ref id) = doc.id {
                span.set_attribute(attributes::reranker::input_documents::id(i), id.clone());
            }
            if !self.config.hide_inputs {
                span.set_attribute(attributes::reranker::input_documents::content(i), doc.content.clone());
            } else {
                span.set_attribute(attributes::reranker::input_documents::content(i), REDACTED);
            }
            if let Some(score) = doc.score {
                span.set_attribute(attributes::reranker::input_documents::score(i), score);
            }
        }

        span
    }
}

/// Record reranker output documents on a span.
pub fn record_reranker_output_documents(
    span: &Span,
    documents: &[Document],
    config: &TraceConfig,
) {
    for (i, doc) in documents.iter().enumerate() {
        if let Some(ref id) = doc.id {
            span.set_attribute(attributes::reranker::output_documents::id(i), id.clone());
        }
        if !config.hide_outputs {
            span.set_attribute(attributes::reranker::output_documents::content(i), doc.content.clone());
        } else {
            span.set_attribute(attributes::reranker::output_documents::content(i), REDACTED);
        }
        if let Some(score) = doc.score {
            span.set_attribute(attributes::reranker::output_documents::score(i), score);
        }
    }
}

// =============================================================================
// Guardrail Span Builder
// =============================================================================

/// Builder for guardrail spans (input/output safety checks).
#[derive(Debug)]
pub struct GuardrailSpanBuilder {
    name: String,
    input_value: Option<String>,
    output_value: Option<String>,
    config: TraceConfig,
}

impl GuardrailSpanBuilder {
    /// Create a new guardrail span builder with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            input_value: None,
            output_value: None,
            config: TraceConfig::default(),
        }
    }

    /// Set the configuration for this builder.
    pub fn config(mut self, config: TraceConfig) -> Self {
        self.config = config;
        self
    }

    /// Set the input value.
    pub fn input_value(mut self, value: impl Into<String>) -> Self {
        self.input_value = Some(value.into());
        self
    }

    /// Set the output value.
    pub fn output_value(mut self, value: impl Into<String>) -> Self {
        self.output_value = Some(value.into());
        self
    }

    /// Build the span.
    pub fn build(self) -> Span {
        let span_name = format!("guardrail {}", self.name);

        let span = tracing::info_span!("guardrail", otel.name = %span_name);

        span.set_attribute(attributes::OPENINFERENCE_SPAN_KIND, SpanKind::Guardrail.as_str());

        if let Some(ref input) = self.input_value {
            if !self.config.hide_inputs {
                span.set_attribute(attributes::input::VALUE, input.clone());
            } else {
                span.set_attribute(attributes::input::VALUE, REDACTED);
            }
        }
        if let Some(ref output) = self.output_value {
            if !self.config.hide_outputs {
                span.set_attribute(attributes::output::VALUE, output.clone());
            } else {
                span.set_attribute(attributes::output::VALUE, REDACTED);
            }
        }

        span
    }
}

// =============================================================================
// Evaluator Span Builder
// =============================================================================

/// Builder for evaluator spans (model output evaluation).
#[derive(Debug)]
pub struct EvaluatorSpanBuilder {
    name: String,
    input_value: Option<String>,
    output_value: Option<String>,
    config: TraceConfig,
}

impl EvaluatorSpanBuilder {
    /// Create a new evaluator span builder with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            input_value: None,
            output_value: None,
            config: TraceConfig::default(),
        }
    }

    /// Set the configuration for this builder.
    pub fn config(mut self, config: TraceConfig) -> Self {
        self.config = config;
        self
    }

    /// Set the input value.
    pub fn input_value(mut self, value: impl Into<String>) -> Self {
        self.input_value = Some(value.into());
        self
    }

    /// Set the output value.
    pub fn output_value(mut self, value: impl Into<String>) -> Self {
        self.output_value = Some(value.into());
        self
    }

    /// Build the span.
    pub fn build(self) -> Span {
        let span_name = format!("evaluator {}", self.name);

        let span = tracing::info_span!("evaluator", otel.name = %span_name);

        span.set_attribute(attributes::OPENINFERENCE_SPAN_KIND, SpanKind::Evaluator.as_str());

        if let Some(ref input) = self.input_value {
            if !self.config.hide_inputs {
                span.set_attribute(attributes::input::VALUE, input.clone());
            } else {
                span.set_attribute(attributes::input::VALUE, REDACTED);
            }
        }
        if let Some(ref output) = self.output_value {
            if !self.config.hide_outputs {
                span.set_attribute(attributes::output::VALUE, output.clone());
            } else {
                span.set_attribute(attributes::output::VALUE, REDACTED);
            }
        }

        span
    }
}

// =============================================================================
// Helper functions for recording attributes post-creation
// =============================================================================

/// Record token usage on a span.
///
/// Records both OpenInference (`llm.token_count.*`) and OTel GenAI
/// (`gen_ai.usage.*`) token count attributes.
pub fn record_token_usage(span: &Span, prompt_tokens: i64, completion_tokens: i64) {
    let total_tokens = prompt_tokens + completion_tokens;

    // OpenInference attributes
    span.set_attribute(attributes::llm::token_count::PROMPT, prompt_tokens);
    span.set_attribute(attributes::llm::token_count::COMPLETION, completion_tokens);
    span.set_attribute(attributes::llm::token_count::TOTAL, total_tokens);

    // OTel GenAI attributes
    span.set_attribute(gen_ai::usage::INPUT_TOKENS, prompt_tokens);
    span.set_attribute(gen_ai::usage::OUTPUT_TOKENS, completion_tokens);
}

/// Record an output message on a span at the given index.
///
/// Supports arbitrary message indices via dynamic attribute keys.
/// Content is subject to `TraceConfig` privacy controls.
pub fn record_output_message(
    span: &Span,
    index: usize,
    role: &str,
    content: &str,
    config: &TraceConfig,
) {
    let hide_messages = config.should_hide_output_messages();
    let hide_text = config.should_hide_output_text();

    if hide_messages {
        span.set_attribute(attributes::llm::output_messages::role(index), REDACTED);
        span.set_attribute(attributes::llm::output_messages::content(index), REDACTED);
    } else {
        span.set_attribute(attributes::llm::output_messages::role(index), role.to_string());
        if hide_text {
            span.set_attribute(attributes::llm::output_messages::content(index), REDACTED);
        } else {
            span.set_attribute(attributes::llm::output_messages::content(index), content.to_string());
        }
    }
}

/// Record a tool call on an output message.
pub fn record_output_tool_call(
    span: &Span,
    message_index: usize,
    call_index: usize,
    tool_call_id: &str,
    function_name: &str,
    function_arguments: &str,
) {
    span.set_attribute(attributes::llm::output_messages::tool_calls::id(message_index, call_index), tool_call_id.to_string());
    span.set_attribute(attributes::llm::output_messages::tool_calls::function_name(message_index, call_index), function_name.to_string());
    span.set_attribute(attributes::llm::output_messages::tool_calls::function_arguments(
            message_index, call_index,
        ),
        function_arguments.to_string());
}

/// Record retrieval documents on a span.
pub fn record_retrieval_documents(span: &Span, documents: &[Document], config: &TraceConfig) {
    for (i, doc) in documents.iter().enumerate() {
        if let Some(ref id) = doc.id {
            span.set_attribute(attributes::retrieval::documents::id(i), id.clone());
        }
        if !config.hide_outputs {
            span.set_attribute(attributes::retrieval::documents::content(i), doc.content.clone());
        } else {
            span.set_attribute(attributes::retrieval::documents::content(i), REDACTED);
        }
        if let Some(score) = doc.score {
            span.set_attribute(attributes::retrieval::documents::score(i), score);
        }
    }
}

/// Record an error on a span.
pub fn record_error(span: &Span, error_type: &str, message: &str) {
    span.set_attribute(attributes::exception::TYPE, error_type.to_string());
    span.set_attribute(attributes::exception::MESSAGE, message.to_string());
}

/// Record the output value on a span.
pub fn record_output_value(span: &Span, value: &str, config: &TraceConfig) {
    if !config.hide_outputs {
        span.set_attribute(attributes::output::VALUE, value.to_string());
    } else {
        span.set_attribute(attributes::output::VALUE, REDACTED);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    /// Initialize a test subscriber that actually processes spans.
    fn init_test_subscriber() {
        let _ = tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().with_test_writer())
            .try_init();
    }

    #[test]
    fn test_llm_span_builder() {
        init_test_subscriber();

        let _span = LlmSpanBuilder::new("gpt-4")
            .provider("openai")
            .temperature(0.7)
            .max_tokens(1000)
            .build();

        // Builder with messages and full config
        let config = TraceConfig::builder()
            .emit_gen_ai_attributes(true)
            .build();
        let _span2 = LlmSpanBuilder::new("claude-3")
            .config(config)
            .provider("anthropic")
            .system("anthropic")
            .top_p(0.9)
            .top_k(50)
            .frequency_penalty(0.5)
            .presence_penalty(0.5)
            .invocation_parameters(r#"{"stream": true}"#)
            .input_message("system", "You are a helpful assistant.")
            .input_message("user", "Hello!")
            .input_value("raw request body")
            .output_value("raw response body")
            .tool(r#"{"name":"calc","description":"calculator"}"#)
            .build();
    }

    #[test]
    fn test_llm_span_builder_privacy() {
        init_test_subscriber();

        let config = TraceConfig::builder()
            .hide_inputs(true)
            .hide_outputs(true)
            .hide_llm_invocation_parameters(true)
            .build();

        let _span = LlmSpanBuilder::new("gpt-4")
            .config(config)
            .input_message("user", "secret prompt")
            .input_value("secret input")
            .output_value("secret output")
            .invocation_parameters(r#"{"secret": true}"#)
            .build();
    }

    #[test]
    fn test_embedding_span_builder() {
        init_test_subscriber();

        let _span = EmbeddingSpanBuilder::new("text-embedding-ada-002")
            .text("Hello, world!")
            .build();

        let _span2 = EmbeddingSpanBuilder::new("embed-v3")
            .texts(vec!["text1", "text2", "text3"])
            .config(TraceConfig::default())
            .build();
    }

    #[test]
    fn test_chain_span_builder() {
        init_test_subscriber();

        let _span = ChainSpanBuilder::new("process_request")
            .input("user query")
            .build();

        let config = TraceConfig::builder()
            .emit_gen_ai_attributes(false)
            .build();
        let _span2 = ChainSpanBuilder::new("rag_pipeline")
            .config(config)
            .input("What is Rust?")
            .input_mime_type("text/plain")
            .output("Rust is a systems programming language.")
            .output_mime_type("text/plain")
            .build();
    }

    #[test]
    fn test_tool_span_builder() {
        init_test_subscriber();

        let _span = ToolSpanBuilder::new("calculator")
            .description("Performs arithmetic calculations")
            .parameters(r#"{"operation": "add", "a": 1, "b": 2}"#)
            .build();

        let _span2 = ToolSpanBuilder::new("web_search")
            .description("Searches the web")
            .config(TraceConfig::default())
            .build();
    }

    #[test]
    fn test_retriever_span_builder() {
        init_test_subscriber();

        let _span = RetrieverSpanBuilder::new("vector_search")
            .query("What is the capital of France?")
            .top_k(5)
            .build();

        let _span2 = RetrieverSpanBuilder::new("pinecone")
            .config(TraceConfig::default())
            .build();
    }

    #[test]
    fn test_agent_span_builder() {
        init_test_subscriber();

        let _span = AgentSpanBuilder::new("research_agent")
            .input_value("Find information about Rust")
            .output_value("Rust is a systems programming language.")
            .build();
    }

    #[test]
    fn test_reranker_span_builder() {
        init_test_subscriber();

        let _span = RerankerSpanBuilder::new("cross-encoder")
            .query("What is Rust?")
            .top_k(3)
            .input_document(Document {
                id: Some("doc1".to_string()),
                content: "Rust is a programming language.".to_string(),
                score: Some(0.9),
            })
            .input_document(Document {
                id: Some("doc2".to_string()),
                content: "Python is a programming language.".to_string(),
                score: Some(0.5),
            })
            .build();
    }

    #[test]
    fn test_guardrail_span_builder() {
        init_test_subscriber();

        let _span = GuardrailSpanBuilder::new("content_filter")
            .input_value("Check this text for safety")
            .output_value("PASS")
            .build();
    }

    #[test]
    fn test_evaluator_span_builder() {
        init_test_subscriber();

        let _span = EvaluatorSpanBuilder::new("relevance_scorer")
            .input_value("Is this response relevant?")
            .output_value("0.95")
            .build();
    }

    #[test]
    fn test_record_token_usage() {
        init_test_subscriber();

        let span = LlmSpanBuilder::new("gpt-4").build();
        record_token_usage(&span, 100, 50);
    }

    #[test]
    fn test_record_output_message() {
        init_test_subscriber();

        let config = TraceConfig::default();
        let span = LlmSpanBuilder::new("gpt-4").build();
        record_output_message(&span, 0, "assistant", "Hello!", &config);
        record_output_message(&span, 1, "assistant", "How can I help?", &config);
    }

    #[test]
    fn test_record_output_message_privacy() {
        init_test_subscriber();

        let config = TraceConfig::builder()
            .hide_output_messages(true)
            .build();
        let span = LlmSpanBuilder::new("gpt-4").build();
        record_output_message(&span, 0, "assistant", "secret", &config);
    }

    #[test]
    fn test_record_output_tool_call() {
        init_test_subscriber();

        let span = LlmSpanBuilder::new("gpt-4").build();
        record_output_tool_call(
            &span,
            0,
            0,
            "call_abc123",
            "get_weather",
            r#"{"location": "Paris"}"#,
        );
    }

    #[test]
    fn test_record_error() {
        init_test_subscriber();

        let span = LlmSpanBuilder::new("gpt-4").build();
        record_error(&span, "RateLimitError", "Too many requests");
    }

    #[test]
    fn test_record_retrieval_documents() {
        init_test_subscriber();

        let config = TraceConfig::default();
        let span = RetrieverSpanBuilder::new("search").build();
        record_retrieval_documents(
            &span,
            &[
                Document {
                    id: Some("doc1".to_string()),
                    content: "First document".to_string(),
                    score: Some(0.95),
                },
                Document {
                    id: None,
                    content: "Second document".to_string(),
                    score: None,
                },
            ],
            &config,
        );
    }

    #[test]
    fn test_record_output_value() {
        init_test_subscriber();

        let config = TraceConfig::default();
        let span = ChainSpanBuilder::new("test").build();
        record_output_value(&span, "the result", &config);

        let hidden_config = TraceConfig::builder().hide_outputs(true).build();
        record_output_value(&span, "secret result", &hidden_config);
    }

    #[test]
    fn test_trace_config_default() {
        let config = TraceConfig::default();
        assert!(config.emit_gen_ai_attributes);
        assert!(!config.hide_inputs);
        assert!(!config.hide_outputs);
    }
}
