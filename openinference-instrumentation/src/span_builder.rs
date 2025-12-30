//! Span builders for OpenInference-compliant instrumentation.
//!
//! These builders provide a fluent API for creating spans with the correct
//! OpenInference attributes, and optionally dual-writing OTel GenAI attributes.

use openinference_semantic_conventions::SpanKind;
use tracing::{span, Level, Span};

/// Configuration for span builders.
#[derive(Debug, Clone)]
pub struct SpanConfig {
    /// Whether to also emit OTel GenAI semantic convention attributes.
    pub emit_gen_ai_attributes: bool,
    /// Whether to record message content (may contain sensitive data).
    pub record_content: bool,
}

impl Default for SpanConfig {
    fn default() -> Self {
        Self {
            emit_gen_ai_attributes: true,
            record_content: false,
        }
    }
}

// =============================================================================
// LLM Span Builder
// =============================================================================

/// Builder for LLM (Large Language Model) spans.
///
/// # Example
///
/// ```rust,ignore
/// let span = LlmSpanBuilder::new("gpt-4")
///     .provider("openai")
///     .temperature(0.7)
///     .max_tokens(1000)
///     .input_message(0, "system", "You are a helpful assistant.")
///     .input_message(1, "user", "Hello!")
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
    input_messages: Vec<(usize, String, String)>, // (index, role, content)
    invocation_parameters: Option<String>,
    config: SpanConfig,
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
            config: SpanConfig::default(),
        }
    }

    /// Set the configuration for this builder.
    pub fn config(mut self, config: SpanConfig) -> Self {
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

    /// Add an input message.
    ///
    /// Content is only recorded if `config.record_content` is true.
    pub fn input_message(
        mut self,
        index: usize,
        role: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        self.input_messages
            .push((index, role.into(), content.into()));
        self
    }

    /// Set the invocation parameters as a JSON string.
    pub fn invocation_parameters(mut self, params: impl Into<String>) -> Self {
        self.invocation_parameters = Some(params.into());
        self
    }

    /// Build the span.
    ///
    /// Returns a `tracing::Span` with all the configured attributes.
    pub fn build(self) -> Span {
        let span_name = format!("llm {}", self.model_name);

        // Create the base span with required attributes
        // Using string literals for field names since tracing requires compile-time constants
        let span = span!(
            Level::INFO,
            "llm",
            otel.name = %span_name,
            "openinference.span.kind" = SpanKind::Llm.as_str(),
            "llm.model_name" = %self.model_name,
            "llm.provider" = tracing::field::Empty,
            "llm.system" = tracing::field::Empty,
            "llm.invocation_parameters" = tracing::field::Empty,
            // OTel GenAI attributes
            "gen_ai.request.model" = %self.model_name,
            "gen_ai.provider.name" = tracing::field::Empty,
            "gen_ai.system" = tracing::field::Empty,
            "gen_ai.request.temperature" = tracing::field::Empty,
            "gen_ai.request.top_p" = tracing::field::Empty,
            "gen_ai.request.top_k" = tracing::field::Empty,
            "gen_ai.request.max_tokens" = tracing::field::Empty,
            "gen_ai.request.frequency_penalty" = tracing::field::Empty,
            "gen_ai.request.presence_penalty" = tracing::field::Empty,
        );

        // Record optional OpenInference attributes
        if let Some(ref provider) = self.provider {
            span.record("llm.provider", provider.as_str());
        }
        if let Some(ref system) = self.system {
            span.record("llm.system", system.as_str());
        }
        if let Some(ref params) = self.invocation_parameters {
            span.record("llm.invocation_parameters", params.as_str());
        }

        // Record OTel GenAI attributes if enabled
        if self.config.emit_gen_ai_attributes {
            if let Some(ref provider) = self.provider {
                span.record("gen_ai.provider.name", provider.as_str());
            }
            if let Some(ref system) = self.system {
                span.record("gen_ai.system", system.as_str());
            }
            if let Some(temp) = self.temperature {
                span.record("gen_ai.request.temperature", temp);
            }
            if let Some(top_p) = self.top_p {
                span.record("gen_ai.request.top_p", top_p);
            }
            if let Some(top_k) = self.top_k {
                span.record("gen_ai.request.top_k", top_k);
            }
            if let Some(max_tokens) = self.max_tokens {
                span.record("gen_ai.request.max_tokens", max_tokens);
            }
            if let Some(freq) = self.frequency_penalty {
                span.record("gen_ai.request.frequency_penalty", freq);
            }
            if let Some(pres) = self.presence_penalty {
                span.record("gen_ai.request.presence_penalty", pres);
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
    config: SpanConfig,
}

impl EmbeddingSpanBuilder {
    /// Create a new embedding span builder with the given model name.
    pub fn new(model_name: impl Into<String>) -> Self {
        Self {
            model_name: model_name.into(),
            texts: Vec::new(),
            config: SpanConfig::default(),
        }
    }

    /// Set the configuration for this builder.
    pub fn config(mut self, config: SpanConfig) -> Self {
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

    /// Build the span.
    pub fn build(self) -> Span {
        let span_name = format!("embedding {}", self.model_name);

        span!(
            Level::INFO,
            "embedding",
            otel.name = %span_name,
            "openinference.span.kind" = SpanKind::Embedding.as_str(),
            "embedding.model_name" = %self.model_name,
        )
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
    config: SpanConfig,
}

impl ChainSpanBuilder {
    /// Create a new chain span builder with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            input_value: None,
            input_mime_type: None,
            config: SpanConfig::default(),
        }
    }

    /// Set the configuration for this builder.
    pub fn config(mut self, config: SpanConfig) -> Self {
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

    /// Build the span.
    pub fn build(self) -> Span {
        let span = span!(
            Level::INFO,
            "chain",
            otel.name = %self.name,
            "openinference.span.kind" = SpanKind::Chain.as_str(),
            "input.value" = tracing::field::Empty,
            "input.mime_type" = tracing::field::Empty,
        );

        if self.config.record_content {
            if let Some(ref input) = self.input_value {
                span.record("input.value", input.as_str());
            }
        }
        if let Some(ref mime_type) = self.input_mime_type {
            span.record("input.mime_type", mime_type.as_str());
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
    config: SpanConfig,
}

impl ToolSpanBuilder {
    /// Create a new tool span builder with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            parameters: None,
            config: SpanConfig::default(),
        }
    }

    /// Set the configuration for this builder.
    pub fn config(mut self, config: SpanConfig) -> Self {
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

    /// Build the span.
    pub fn build(self) -> Span {
        let span_name = format!("tool {}", self.name);

        let span = span!(
            Level::INFO,
            "tool",
            otel.name = %span_name,
            "openinference.span.kind" = SpanKind::Tool.as_str(),
            "tool.name" = %self.name,
            "tool.description" = tracing::field::Empty,
            "tool.parameters" = tracing::field::Empty,
        );

        if let Some(ref desc) = self.description {
            span.record("tool.description", desc.as_str());
        }
        if let Some(ref params) = self.parameters {
            span.record("tool.parameters", params.as_str());
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
    config: SpanConfig,
}

impl RetrieverSpanBuilder {
    /// Create a new retriever span builder with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            query: None,
            top_k: None,
            config: SpanConfig::default(),
        }
    }

    /// Set the configuration for this builder.
    pub fn config(mut self, config: SpanConfig) -> Self {
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

        let span = span!(
            Level::INFO,
            "retriever",
            otel.name = %span_name,
            "openinference.span.kind" = SpanKind::Retriever.as_str(),
            "input.value" = tracing::field::Empty,
        );

        if self.config.record_content {
            if let Some(ref query) = self.query {
                span.record("input.value", query.as_str());
            }
        }

        span
    }
}

// =============================================================================
// Helper functions for recording token usage
// =============================================================================

/// Record token usage on a span.
///
/// This records both OpenInference and OTel GenAI token count attributes.
/// Note: The span must have been created with these fields declared as Empty.
pub fn record_token_usage(span: &Span, prompt_tokens: i64, completion_tokens: i64) {
    let total_tokens = prompt_tokens + completion_tokens;

    // OpenInference attributes
    span.record("llm.token_count.prompt", prompt_tokens);
    span.record("llm.token_count.completion", completion_tokens);
    span.record("llm.token_count.total", total_tokens);

    // OTel GenAI attributes
    span.record("gen_ai.usage.input_tokens", prompt_tokens);
    span.record("gen_ai.usage.output_tokens", completion_tokens);
}

/// Record an output message on a span.
///
/// Note: Due to tracing's static field requirements, only the first message (index 0)
/// is supported. For multiple messages, consider using span events instead.
pub fn record_output_message(
    span: &Span,
    _index: usize,
    role: &str,
    content: &str,
    record_content: bool,
) {
    // Tracing requires compile-time field names, so we only support index 0
    span.record("llm.output_messages.0.message.role", role);
    if record_content {
        span.record("llm.output_messages.0.message.content", content);
    }
}

/// Record an error on a span.
pub fn record_error(span: &Span, error_type: &str, message: &str) {
    span.record("exception.type", error_type);
    span.record("exception.message", message);
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

        // Test that the builder can be constructed and build() doesn't panic
        let _span = LlmSpanBuilder::new("gpt-4")
            .provider("openai")
            .temperature(0.7)
            .max_tokens(1000)
            .build();

        // Builder pattern works
        let _span2 = LlmSpanBuilder::new("claude-3")
            .provider("anthropic")
            .system("anthropic")
            .top_p(0.9)
            .top_k(50)
            .frequency_penalty(0.5)
            .presence_penalty(0.5)
            .invocation_parameters(r#"{"stream": true}"#)
            .config(SpanConfig {
                emit_gen_ai_attributes: true,
                record_content: true,
            })
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
            .config(SpanConfig::default())
            .build();
    }

    #[test]
    fn test_chain_span_builder() {
        init_test_subscriber();

        let _span = ChainSpanBuilder::new("process_request")
            .input("user query")
            .build();

        let _span2 = ChainSpanBuilder::new("rag_pipeline")
            .input("What is Rust?")
            .input_mime_type("text/plain")
            .config(SpanConfig {
                emit_gen_ai_attributes: false,
                record_content: true,
            })
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
            .config(SpanConfig::default())
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
            .config(SpanConfig {
                emit_gen_ai_attributes: true,
                record_content: true,
            })
            .build();
    }

    #[test]
    fn test_span_config_default() {
        let config = SpanConfig::default();
        assert!(config.emit_gen_ai_attributes);
        assert!(!config.record_content);
    }
}
