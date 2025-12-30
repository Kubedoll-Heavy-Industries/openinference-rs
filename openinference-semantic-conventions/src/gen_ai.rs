//! OpenTelemetry GenAI Semantic Conventions.
//!
//! This module provides attribute keys compatible with the
//! [OTel GenAI semantic conventions](https://opentelemetry.io/docs/specs/semconv/gen-ai/).
//!
//! These can be used alongside OpenInference attributes for maximum compatibility
//! with observability backends like Datadog, Honeycomb, and others that support
//! the OTel standard.

use opentelemetry::Key;

// =============================================================================
// Core GenAI Attributes
// =============================================================================

/// The name of the operation being performed (e.g., "chat", "text_completion").
pub const OPERATION_NAME: Key = Key::from_static_str("gen_ai.operation.name");

/// The provider of the GenAI system (e.g., "openai", "anthropic", "mistral.rs").
pub const PROVIDER_NAME: Key = Key::from_static_str("gen_ai.provider.name");

/// The name of the GenAI system (e.g., "openai", "anthropic").
pub const SYSTEM: Key = Key::from_static_str("gen_ai.system");

// =============================================================================
// Request Attributes
// =============================================================================

/// Attributes for GenAI requests.
pub mod request {
    use opentelemetry::Key;

    /// The name of the model being requested.
    pub const MODEL: Key = Key::from_static_str("gen_ai.request.model");

    /// The temperature parameter for generation.
    pub const TEMPERATURE: Key = Key::from_static_str("gen_ai.request.temperature");

    /// The top_p (nucleus sampling) parameter.
    pub const TOP_P: Key = Key::from_static_str("gen_ai.request.top_p");

    /// The top_k parameter.
    pub const TOP_K: Key = Key::from_static_str("gen_ai.request.top_k");

    /// Maximum number of tokens to generate.
    pub const MAX_TOKENS: Key = Key::from_static_str("gen_ai.request.max_tokens");

    /// Stop sequences for generation.
    pub const STOP_SEQUENCES: Key = Key::from_static_str("gen_ai.request.stop_sequences");

    /// Frequency penalty.
    pub const FREQUENCY_PENALTY: Key = Key::from_static_str("gen_ai.request.frequency_penalty");

    /// Presence penalty.
    pub const PRESENCE_PENALTY: Key = Key::from_static_str("gen_ai.request.presence_penalty");

    /// Finish reasons requested.
    pub const FINISH_REASONS: Key = Key::from_static_str("gen_ai.request.finish_reasons");

    /// System instructions/prompt.
    pub const SYSTEM_INSTRUCTIONS: Key = Key::from_static_str("gen_ai.system_instructions");

    /// Input messages (for content recording, opt-in).
    pub const INPUT_MESSAGES: Key = Key::from_static_str("gen_ai.input.messages");
}

// =============================================================================
// Response Attributes
// =============================================================================

/// Attributes for GenAI responses.
pub mod response {
    use opentelemetry::Key;

    /// The model that actually generated the response.
    pub const MODEL: Key = Key::from_static_str("gen_ai.response.model");

    /// The ID of the response.
    pub const ID: Key = Key::from_static_str("gen_ai.response.id");

    /// Finish reasons for the response.
    pub const FINISH_REASONS: Key = Key::from_static_str("gen_ai.response.finish_reasons");

    /// Output messages (for content recording, opt-in).
    pub const OUTPUT_MESSAGES: Key = Key::from_static_str("gen_ai.output.messages");
}

// =============================================================================
// Usage Attributes
// =============================================================================

/// Token usage attributes.
pub mod usage {
    use opentelemetry::Key;

    /// Number of input tokens used.
    pub const INPUT_TOKENS: Key = Key::from_static_str("gen_ai.usage.input_tokens");

    /// Number of output tokens generated.
    pub const OUTPUT_TOKENS: Key = Key::from_static_str("gen_ai.usage.output_tokens");
}

// =============================================================================
// Token Attributes (for events)
// =============================================================================

/// Token-level attributes for streaming events.
pub mod token {
    use opentelemetry::Key;

    /// Token type (e.g., "input", "output").
    pub const TYPE: Key = Key::from_static_str("gen_ai.token.type");
}

// =============================================================================
// Choice Attributes
// =============================================================================

/// Choice attributes for multi-choice responses.
pub mod choice {
    use opentelemetry::Key;

    /// Choice finish reason.
    pub const FINISH_REASON: Key = Key::from_static_str("gen_ai.choice.finish_reason");

    /// Choice index.
    pub const INDEX: Key = Key::from_static_str("gen_ai.choice.index");
}

// =============================================================================
// Prompt Attributes
// =============================================================================

/// Prompt attributes.
pub mod prompt {
    use opentelemetry::Key;

    /// Prompt template used.
    pub const TEMPLATE: Key = Key::from_static_str("gen_ai.prompt.template");

    /// Prompt template version.
    pub const VERSION: Key = Key::from_static_str("gen_ai.prompt.version");
}

// =============================================================================
// Tool Attributes
// =============================================================================

/// Tool attributes for function calling.
pub mod tool {
    use opentelemetry::Key;

    /// Tool name.
    pub const NAME: Key = Key::from_static_str("gen_ai.tool.name");

    /// Tool call ID.
    pub const CALL_ID: Key = Key::from_static_str("gen_ai.tool.call.id");

    /// Tool arguments.
    pub const ARGUMENTS: Key = Key::from_static_str("gen_ai.tool.arguments");

    /// Tool result.
    pub const RESULT: Key = Key::from_static_str("gen_ai.tool.result");
}

// =============================================================================
// Agent Attributes
// =============================================================================

/// Agent attributes.
pub mod agent {
    use opentelemetry::Key;

    /// Agent name.
    pub const NAME: Key = Key::from_static_str("gen_ai.agent.name");

    /// Agent description.
    pub const DESCRIPTION: Key = Key::from_static_str("gen_ai.agent.description");

    /// Agent ID.
    pub const ID: Key = Key::from_static_str("gen_ai.agent.id");
}

// =============================================================================
// Event Names
// =============================================================================

/// Standard event names for GenAI operations.
pub mod events {
    /// Event name for content generation.
    pub const CONTENT: &str = "gen_ai.content";

    /// Event name for tool calls.
    pub const TOOL_CALL: &str = "gen_ai.tool.call";

    /// Event name for choice events.
    pub const CHOICE: &str = "gen_ai.choice";

    /// Event name for system prompt.
    pub const SYSTEM_PROMPT: &str = "gen_ai.system.prompt";

    /// Event name for user prompt.
    pub const USER_PROMPT: &str = "gen_ai.user.prompt";

    /// Event name for assistant response.
    pub const ASSISTANT_RESPONSE: &str = "gen_ai.assistant.response";
}

// =============================================================================
// Metric Names
// =============================================================================

/// Standard metric names for GenAI operations.
pub mod metrics {
    /// Counter for client requests duration.
    pub const CLIENT_REQUEST_DURATION: &str = "gen_ai.client.request.duration";

    /// Counter for client token usage.
    pub const CLIENT_TOKEN_USAGE: &str = "gen_ai.client.token.usage";

    /// Counter for server request duration.
    pub const SERVER_REQUEST_DURATION: &str = "gen_ai.server.request.duration";

    /// Counter for server time to first token.
    pub const SERVER_TIME_TO_FIRST_TOKEN: &str = "gen_ai.server.time_to_first_token";

    /// Counter for server time per output token.
    pub const SERVER_TIME_PER_OUTPUT_TOKEN: &str = "gen_ai.server.time_per_output_token";
}

// =============================================================================
// Helpers for mapping between OpenInference and OTel GenAI
// =============================================================================

/// Maps an OpenInference attribute key to its OTel GenAI equivalent, if one exists.
///
/// Returns `None` if there is no direct mapping.
pub fn map_openinference_to_gen_ai(openinference_key: &str) -> Option<Key> {
    match openinference_key {
        "llm.model_name" => Some(request::MODEL),
        "llm.provider" => Some(PROVIDER_NAME),
        "llm.system" => Some(SYSTEM),
        "llm.token_count.prompt" => Some(usage::INPUT_TOKENS),
        "llm.token_count.completion" => Some(usage::OUTPUT_TOKENS),
        _ => None,
    }
}

/// Maps an OTel GenAI attribute key to its OpenInference equivalent, if one exists.
pub fn map_gen_ai_to_openinference(gen_ai_key: &str) -> Option<Key> {
    match gen_ai_key {
        "gen_ai.request.model" => Some(crate::attributes::llm::MODEL_NAME),
        "gen_ai.provider.name" => Some(crate::attributes::llm::PROVIDER),
        "gen_ai.system" => Some(crate::attributes::llm::SYSTEM),
        "gen_ai.usage.input_tokens" => Some(crate::attributes::llm::token_count::PROMPT),
        "gen_ai.usage.output_tokens" => Some(crate::attributes::llm::token_count::COMPLETION),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribute_mapping_roundtrip() {
        // OpenInference -> GenAI
        assert_eq!(
            map_openinference_to_gen_ai("llm.model_name"),
            Some(request::MODEL)
        );
        assert_eq!(
            map_openinference_to_gen_ai("llm.token_count.prompt"),
            Some(usage::INPUT_TOKENS)
        );

        // GenAI -> OpenInference
        assert_eq!(
            map_gen_ai_to_openinference("gen_ai.request.model"),
            Some(crate::attributes::llm::MODEL_NAME)
        );
    }

    #[test]
    fn test_unknown_attributes_return_none() {
        assert_eq!(map_openinference_to_gen_ai("unknown.attribute"), None);
        assert_eq!(map_gen_ai_to_openinference("unknown.attribute"), None);
    }
}
