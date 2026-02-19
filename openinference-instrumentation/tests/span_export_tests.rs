//! Integration tests that wire up a real OTel pipeline with `InMemorySpanExporter`
//! and verify that exported spans have the correct OpenInference and GenAI attributes.

use opentelemetry::trace::TracerProvider as _;
use opentelemetry::Value;
use opentelemetry_sdk::trace::{InMemorySpanExporterBuilder, SdkTracerProvider, SpanData};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

use openinference_instrumentation::span_builder::{
    ChainSpanBuilder, EmbeddingSpanBuilder, LlmSpanBuilder, RetrieverSpanBuilder,
    ToolSpanBuilder,
};
use openinference_instrumentation::TraceConfig;

// =============================================================================
// Test harness
// =============================================================================

/// Sets up a tracing subscriber backed by an in-memory OTel exporter.
///
/// Returns the subscriber, the exporter handle (for inspecting spans), and the
/// provider (which must be kept alive for the duration of the test).
fn setup_tracing() -> (
    impl tracing::Subscriber,
    opentelemetry_sdk::trace::InMemorySpanExporter,
    SdkTracerProvider,
) {
    let exporter = InMemorySpanExporterBuilder::new().build();
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let tracer = provider.tracer("test");
    let telemetry = OpenTelemetryLayer::new(tracer);
    let subscriber = Registry::default().with(telemetry);
    (subscriber, exporter, provider)
}

/// Find an attribute value in an exported span by key name.
fn find_attribute(span: &SpanData, key: &str) -> Option<Value> {
    span.attributes
        .iter()
        .find(|kv| kv.key.as_str() == key)
        .map(|kv| kv.value.clone())
}

/// Assert that a span contains an attribute with the given string value.
fn assert_string_attribute(span: &SpanData, key: &str, expected: &str) {
    let val = find_attribute(span, key).unwrap_or_else(|| {
        panic!(
            "attribute '{}' not found in span. attributes: {:?}",
            key, span.attributes
        )
    });
    match &val {
        Value::String(s) => assert_eq!(
            s.as_str(),
            expected,
            "attribute '{}' expected '{}', got '{}'",
            key,
            expected,
            s.as_str()
        ),
        other => panic!(
            "attribute '{}' expected String('{}'), got {:?}",
            key, expected, other
        ),
    }
}

/// Assert that a span contains an attribute with the given i64 value.
fn assert_i64_attribute(span: &SpanData, key: &str, expected: i64) {
    let val = find_attribute(span, key).unwrap_or_else(|| {
        panic!(
            "attribute '{}' not found in span. attributes: {:?}",
            key, span.attributes
        )
    });
    match &val {
        Value::I64(v) => assert_eq!(
            *v, expected,
            "attribute '{}' expected {}, got {}",
            key, expected, v
        ),
        other => panic!(
            "attribute '{}' expected I64({}), got {:?}",
            key, expected, other
        ),
    }
}

/// Assert that a span contains an attribute with the given f64 value.
fn assert_f64_attribute(span: &SpanData, key: &str, expected: f64) {
    let val = find_attribute(span, key).unwrap_or_else(|| {
        panic!(
            "attribute '{}' not found in span. attributes: {:?}",
            key, span.attributes
        )
    });
    match &val {
        Value::F64(v) => assert!(
            (*v - expected).abs() < f64::EPSILON,
            "attribute '{}' expected {}, got {}",
            key,
            expected,
            v
        ),
        other => panic!(
            "attribute '{}' expected F64({}), got {:?}",
            key, expected, other
        ),
    }
}

/// Assert that a span does NOT contain an attribute with the given key.
fn assert_no_attribute(span: &SpanData, key: &str) {
    if let Some(val) = find_attribute(span, key) {
        panic!(
            "attribute '{}' should NOT be present in span, but found {:?}",
            key, val
        );
    }
}

// =============================================================================
// LLM span tests
// =============================================================================

#[test]
fn test_llm_span_attributes() {
    let (subscriber, exporter, _provider) = setup_tracing();

    tracing::subscriber::with_default(subscriber, || {
        let span = LlmSpanBuilder::new("gpt-4")
            .provider("openai")
            .system("openai")
            .temperature(0.7)
            .top_p(0.9)
            .max_tokens(1000)
            .frequency_penalty(0.5)
            .presence_penalty(0.3)
            .build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1, "expected exactly 1 span, got {}", spans.len());
    let span = &spans[0];

    // Core OpenInference attributes
    assert_string_attribute(span, "openinference.span.kind", "LLM");
    assert_string_attribute(span, "llm.model_name", "gpt-4");
    assert_string_attribute(span, "llm.provider", "openai");
    assert_string_attribute(span, "llm.system", "openai");

    // OTel GenAI attributes (dual emission enabled by default)
    assert_string_attribute(span, "gen_ai.request.model", "gpt-4");
    assert_string_attribute(span, "gen_ai.provider.name", "openai");
    assert_string_attribute(span, "gen_ai.system", "openai");
    assert_f64_attribute(span, "gen_ai.request.temperature", 0.7);
    assert_f64_attribute(span, "gen_ai.request.top_p", 0.9);
    assert_i64_attribute(span, "gen_ai.request.max_tokens", 1000);
    assert_f64_attribute(span, "gen_ai.request.frequency_penalty", 0.5);
    assert_f64_attribute(span, "gen_ai.request.presence_penalty", 0.3);
}

#[test]
fn test_llm_input_messages() {
    let (subscriber, exporter, _provider) = setup_tracing();

    let config = TraceConfig::default();

    tracing::subscriber::with_default(subscriber, || {
        let span = LlmSpanBuilder::new("gpt-4")
            .config(config)
            .input_message("system", "You are a helpful assistant.")
            .input_message("user", "Hello!")
            .build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    let span = &spans[0];

    assert_string_attribute(span, "openinference.span.kind", "LLM");
    assert_string_attribute(span, "llm.model_name", "gpt-4");

    // Input messages are emitted as dynamic OTel attributes via set_attribute()
    assert_string_attribute(span, "llm.input_messages.0.message.role", "system");
    assert_string_attribute(span, "llm.input_messages.0.message.content", "You are a helpful assistant.");
    assert_string_attribute(span, "llm.input_messages.1.message.role", "user");
    assert_string_attribute(span, "llm.input_messages.1.message.content", "Hello!");
}

#[test]
fn test_token_usage_recording() {
    let (subscriber, exporter, _provider) = setup_tracing();

    tracing::subscriber::with_default(subscriber, || {
        // The current LlmSpanBuilder does NOT declare token count fields in the
        // span!() macro, so record_token_usage() calls span.record() on
        // undeclared fields -- which is silently ignored by tracing.
        let span = LlmSpanBuilder::new("gpt-4").build();
        openinference_instrumentation::span_builder::record_token_usage(&span, 100, 50);
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    let span = &spans[0];

    assert_string_attribute(span, "openinference.span.kind", "LLM");

    // Token usage attributes are now emitted via set_attribute()
    assert_i64_attribute(span, "llm.token_count.prompt", 100);
    assert_i64_attribute(span, "llm.token_count.completion", 50);
    assert_i64_attribute(span, "llm.token_count.total", 150);
    assert_i64_attribute(span, "gen_ai.usage.input_tokens", 100);
    assert_i64_attribute(span, "gen_ai.usage.output_tokens", 50);
}

// =============================================================================
// Privacy / hide_inputs tests
// =============================================================================

#[test]
fn test_privacy_hides_content() {
    let (subscriber, exporter, _provider) = setup_tracing();

    let config = TraceConfig::builder().hide_inputs(true).build();

    tracing::subscriber::with_default(subscriber, || {
        let span = ChainSpanBuilder::new("private_chain")
            .config(config)
            .input("this is sensitive input")
            .build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    let span = &spans[0];

    assert_string_attribute(span, "openinference.span.kind", "CHAIN");
    // input.value should be redacted because hide_inputs is true
    assert_string_attribute(span, "input.value", "__REDACTED__");
}

#[test]
fn test_privacy_shows_content_when_not_hidden() {
    let (subscriber, exporter, _provider) = setup_tracing();

    // Default config: hide_inputs=false
    let config = TraceConfig::default();

    tracing::subscriber::with_default(subscriber, || {
        let span = ChainSpanBuilder::new("public_chain")
            .config(config)
            .input("this is public input")
            .build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    let span = &spans[0];

    assert_string_attribute(span, "openinference.span.kind", "CHAIN");
    assert_string_attribute(span, "input.value", "this is public input");
}

// =============================================================================
// Embedding span tests
// =============================================================================

#[test]
fn test_embedding_span_attributes() {
    let (subscriber, exporter, _provider) = setup_tracing();

    tracing::subscriber::with_default(subscriber, || {
        let span = EmbeddingSpanBuilder::new("text-embedding-ada-002")
            .text("Hello, world!")
            .build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    let span = &spans[0];

    assert_string_attribute(span, "openinference.span.kind", "EMBEDDING");
    assert_string_attribute(span, "embedding.model_name", "text-embedding-ada-002");
}

// =============================================================================
// Chain span tests
// =============================================================================

#[test]
fn test_chain_span_attributes() {
    let (subscriber, exporter, _provider) = setup_tracing();

    // Default config does NOT hide inputs
    let config = TraceConfig::default();

    tracing::subscriber::with_default(subscriber, || {
        let span = ChainSpanBuilder::new("rag_pipeline")
            .config(config)
            .input("What is Rust?")
            .input_mime_type("text/plain")
            .build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    let span = &spans[0];

    assert_string_attribute(span, "openinference.span.kind", "CHAIN");
    assert_string_attribute(span, "input.value", "What is Rust?");
    assert_string_attribute(span, "input.mime_type", "text/plain");
}

// =============================================================================
// Tool span tests
// =============================================================================

#[test]
fn test_tool_span_attributes() {
    let (subscriber, exporter, _provider) = setup_tracing();

    tracing::subscriber::with_default(subscriber, || {
        let span = ToolSpanBuilder::new("calculator")
            .description("Performs arithmetic calculations")
            .parameters(r#"{"operation": "add", "a": 1, "b": 2}"#)
            .build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    let span = &spans[0];

    assert_string_attribute(span, "openinference.span.kind", "TOOL");
    assert_string_attribute(span, "tool.name", "calculator");
    assert_string_attribute(span, "tool.description", "Performs arithmetic calculations");
    assert_string_attribute(
        span,
        "tool.parameters",
        r#"{"operation": "add", "a": 1, "b": 2}"#,
    );
}

// =============================================================================
// Retriever span tests
// =============================================================================

#[test]
fn test_retriever_span_attributes() {
    let (subscriber, exporter, _provider) = setup_tracing();

    // Default config does NOT hide inputs
    let config = TraceConfig::default();

    tracing::subscriber::with_default(subscriber, || {
        let span = RetrieverSpanBuilder::new("vector_search")
            .config(config)
            .query("What is the capital of France?")
            .build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    let span = &spans[0];

    assert_string_attribute(span, "openinference.span.kind", "RETRIEVER");
    assert_string_attribute(span, "input.value", "What is the capital of France?");
}

#[test]
fn test_retriever_privacy_hides_query() {
    let (subscriber, exporter, _provider) = setup_tracing();

    let config = TraceConfig::builder().hide_inputs(true).build();

    tracing::subscriber::with_default(subscriber, || {
        let span = RetrieverSpanBuilder::new("vector_search")
            .config(config)
            .query("sensitive query")
            .build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    let span = &spans[0];

    assert_string_attribute(span, "openinference.span.kind", "RETRIEVER");
    // input.value should be redacted because hide_inputs is true
    assert_string_attribute(span, "input.value", "__REDACTED__");
}

// =============================================================================
// Dual attribute emission tests
// =============================================================================

#[test]
fn test_dual_attribute_emission_enabled() {
    let (subscriber, exporter, _provider) = setup_tracing();

    let config = TraceConfig::builder().emit_gen_ai_attributes(true).build();

    tracing::subscriber::with_default(subscriber, || {
        let span = LlmSpanBuilder::new("gpt-4")
            .config(config)
            .provider("openai")
            .temperature(0.7)
            .build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    let span = &spans[0];

    // OpenInference attributes should be present
    assert_string_attribute(span, "openinference.span.kind", "LLM");
    assert_string_attribute(span, "llm.model_name", "gpt-4");
    assert_string_attribute(span, "llm.provider", "openai");

    // GenAI attributes should ALSO be present (dual emission)
    assert_string_attribute(span, "gen_ai.request.model", "gpt-4");
    assert_string_attribute(span, "gen_ai.provider.name", "openai");
    assert_f64_attribute(span, "gen_ai.request.temperature", 0.7);
}

#[test]
fn test_dual_attribute_emission_disabled() {
    let (subscriber, exporter, _provider) = setup_tracing();

    let config = TraceConfig::builder()
        .emit_gen_ai_attributes(false)
        .build();

    tracing::subscriber::with_default(subscriber, || {
        let span = LlmSpanBuilder::new("gpt-4")
            .config(config)
            .provider("openai")
            .temperature(0.7)
            .build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    let span = &spans[0];

    // OpenInference attributes should be present
    assert_string_attribute(span, "openinference.span.kind", "LLM");
    assert_string_attribute(span, "llm.model_name", "gpt-4");
    assert_string_attribute(span, "llm.provider", "openai");

    // All GenAI attributes should NOT be present because emit_gen_ai_attributes is false
    assert_no_attribute(span, "gen_ai.request.model");
    assert_no_attribute(span, "gen_ai.provider.name");
    assert_no_attribute(span, "gen_ai.system");
    assert_no_attribute(span, "gen_ai.request.temperature");
    assert_no_attribute(span, "gen_ai.request.top_p");
    assert_no_attribute(span, "gen_ai.request.max_tokens");
    assert_no_attribute(span, "gen_ai.request.frequency_penalty");
    assert_no_attribute(span, "gen_ai.request.presence_penalty");
}

// =============================================================================
// Span name format tests
// =============================================================================

#[test]
fn test_llm_span_name_format() {
    let (subscriber, exporter, _provider) = setup_tracing();

    tracing::subscriber::with_default(subscriber, || {
        let span = LlmSpanBuilder::new("gpt-4").build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].name, "llm gpt-4");
}

#[test]
fn test_embedding_span_name_format() {
    let (subscriber, exporter, _provider) = setup_tracing();

    tracing::subscriber::with_default(subscriber, || {
        let span = EmbeddingSpanBuilder::new("ada-002").build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].name, "embedding ada-002");
}

#[test]
fn test_tool_span_name_format() {
    let (subscriber, exporter, _provider) = setup_tracing();

    tracing::subscriber::with_default(subscriber, || {
        let span = ToolSpanBuilder::new("calculator").build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].name, "tool calculator");
}

#[test]
fn test_retriever_span_name_format() {
    let (subscriber, exporter, _provider) = setup_tracing();

    tracing::subscriber::with_default(subscriber, || {
        let span = RetrieverSpanBuilder::new("pinecone").build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].name, "retriever pinecone");
}

// =============================================================================
// Invocation parameters test
// =============================================================================

#[test]
fn test_llm_span_with_invocation_parameters() {
    let (subscriber, exporter, _provider) = setup_tracing();

    tracing::subscriber::with_default(subscriber, || {
        let span = LlmSpanBuilder::new("claude-3")
            .provider("anthropic")
            .invocation_parameters(r#"{"stream": true, "max_tokens": 4096}"#)
            .build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    let span = &spans[0];

    assert_string_attribute(span, "openinference.span.kind", "LLM");
    assert_string_attribute(span, "llm.model_name", "claude-3");
    assert_string_attribute(span, "llm.provider", "anthropic");
    assert_string_attribute(
        span,
        "llm.invocation_parameters",
        r#"{"stream": true, "max_tokens": 4096}"#,
    );
}

// =============================================================================
// Chain span hides mime_type when hide_inputs is set
// =============================================================================

#[test]
fn test_chain_hide_inputs_hides_mime_type() {
    let (subscriber, exporter, _provider) = setup_tracing();

    let config = TraceConfig::builder().hide_inputs(true).build();

    tracing::subscriber::with_default(subscriber, || {
        let span = ChainSpanBuilder::new("pipeline")
            .config(config)
            .input("sensitive data")
            .input_mime_type("application/json")
            .build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    let span = &spans[0];

    assert_string_attribute(span, "openinference.span.kind", "CHAIN");
    // input.value should be redacted, but input.mime_type is non-sensitive metadata
    assert_string_attribute(span, "input.value", "__REDACTED__");
    assert_string_attribute(span, "input.mime_type", "application/json");
}

// =============================================================================
// Input message privacy tests
// =============================================================================

#[test]
fn test_llm_input_messages_hidden() {
    let (subscriber, exporter, _provider) = setup_tracing();

    let config = TraceConfig::builder().hide_input_messages(true).build();

    tracing::subscriber::with_default(subscriber, || {
        let span = LlmSpanBuilder::new("gpt-4")
            .config(config)
            .input_message("system", "Secret system prompt")
            .input_message("user", "Secret user message")
            .build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    let span = &spans[0];

    // Messages should be redacted when hide_input_messages is true
    assert_string_attribute(span, "llm.input_messages.0.message.role", "__REDACTED__");
    assert_string_attribute(span, "llm.input_messages.0.message.content", "__REDACTED__");
    assert_string_attribute(span, "llm.input_messages.1.message.role", "__REDACTED__");
    assert_string_attribute(span, "llm.input_messages.1.message.content", "__REDACTED__");
}

#[test]
fn test_llm_input_text_hidden_but_role_visible() {
    let (subscriber, exporter, _provider) = setup_tracing();

    // hide_input_text hides content but NOT roles (roles are not considered text)
    let config = TraceConfig::builder().hide_input_text(true).build();

    tracing::subscriber::with_default(subscriber, || {
        let span = LlmSpanBuilder::new("gpt-4")
            .config(config)
            .input_message("system", "Secret content")
            .build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    let span = &spans[0];

    // Role should be visible, content should be redacted
    assert_string_attribute(span, "llm.input_messages.0.message.role", "system");
    assert_string_attribute(span, "llm.input_messages.0.message.content", "__REDACTED__");
}

// =============================================================================
// Output message recording tests
// =============================================================================

#[test]
fn test_record_output_message() {
    let (subscriber, exporter, _provider) = setup_tracing();

    let config = TraceConfig::default();

    tracing::subscriber::with_default(subscriber, || {
        let span = LlmSpanBuilder::new("gpt-4").build();
        openinference_instrumentation::span_builder::record_output_message(
            &span,
            0,
            "assistant",
            "Hello! How can I help?",
            &config,
        );
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    let span = &spans[0];

    assert_string_attribute(span, "llm.output_messages.0.message.role", "assistant");
    assert_string_attribute(
        span,
        "llm.output_messages.0.message.content",
        "Hello! How can I help?",
    );
}

#[test]
fn test_record_output_message_hidden() {
    let (subscriber, exporter, _provider) = setup_tracing();

    let config = TraceConfig::builder()
        .hide_output_messages(true)
        .build();

    tracing::subscriber::with_default(subscriber, || {
        let span = LlmSpanBuilder::new("gpt-4").build();
        openinference_instrumentation::span_builder::record_output_message(
            &span,
            0,
            "assistant",
            "secret response",
            &config,
        );
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    let span = &spans[0];

    // Both role and content should be redacted
    assert_string_attribute(span, "llm.output_messages.0.message.role", "__REDACTED__");
    assert_string_attribute(
        span,
        "llm.output_messages.0.message.content",
        "__REDACTED__",
    );
}

// =============================================================================
// Error recording test
// =============================================================================

#[test]
fn test_record_error() {
    let (subscriber, exporter, _provider) = setup_tracing();

    tracing::subscriber::with_default(subscriber, || {
        let span = LlmSpanBuilder::new("gpt-4").build();
        openinference_instrumentation::span_builder::record_error(
            &span,
            "RateLimitError",
            "Too many requests",
        );
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    let span = &spans[0];

    assert_string_attribute(span, "exception.type", "RateLimitError");
    assert_string_attribute(span, "exception.message", "Too many requests");
}

// =============================================================================
// Embedding text hidden test
// =============================================================================

#[test]
fn test_embedding_text_hidden() {
    let (subscriber, exporter, _provider) = setup_tracing();

    let config = TraceConfig::builder().hide_embeddings_text(true).build();

    tracing::subscriber::with_default(subscriber, || {
        let span = EmbeddingSpanBuilder::new("ada-002")
            .config(config)
            .text("sensitive text to embed")
            .build();
        drop(span);
    });

    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    let span = &spans[0];

    assert_string_attribute(span, "openinference.span.kind", "EMBEDDING");
    assert_string_attribute(span, "embedding.model_name", "ada-002");
    // Text should be redacted
    assert_string_attribute(
        span,
        "embedding.embeddings.0.embedding.text",
        "__REDACTED__",
    );
}
