//! OpenInference semantic convention attribute keys.
//!
//! These constants define the attribute keys used in OpenInference spans.
//! See: <https://github.com/Arize-ai/openinference/blob/main/spec/semantic_conventions.md>

use opentelemetry::Key;

// =============================================================================
// Core Attributes
// =============================================================================

/// The kind of span (LLM, EMBEDDING, CHAIN, TOOL, AGENT, RETRIEVER, RERANKER, GUARDRAIL, EVALUATOR).
/// This attribute is required for all OpenInference spans.
pub const OPENINFERENCE_SPAN_KIND: Key = Key::from_static_str("openinference.span.kind");

// =============================================================================
// LLM Attributes
// =============================================================================

/// Attributes for Large Language Model spans.
pub mod llm {
    use opentelemetry::Key;

    /// The name of the language model being used.
    pub const MODEL_NAME: Key = Key::from_static_str("llm.model_name");

    /// The LLM system or provider (e.g., "openai", "anthropic").
    pub const SYSTEM: Key = Key::from_static_str("llm.system");

    /// The LLM provider name.
    pub const PROVIDER: Key = Key::from_static_str("llm.provider");

    /// JSON string of invocation parameters (temperature, max_tokens, etc.).
    pub const INVOCATION_PARAMETERS: Key = Key::from_static_str("llm.invocation_parameters");

    /// Deprecated function call (use tool_calls instead).
    pub const FUNCTION_CALL: Key = Key::from_static_str("llm.function_call");

    /// Input messages to the LLM.
    pub mod input_messages {
        use opentelemetry::Key;

        /// Format: llm.input_messages.{index}.message.role
        pub fn role(index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("llm.input_messages.{index}.message.role").into_boxed_str(),
            ))
        }

        /// Format: llm.input_messages.{index}.message.content
        pub fn content(index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("llm.input_messages.{index}.message.content").into_boxed_str(),
            ))
        }

        /// Format: llm.input_messages.{index}.message.contents.{content_index}.message_content.type
        pub fn content_type(index: usize, content_index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("llm.input_messages.{index}.message.contents.{content_index}.message_content.type").into_boxed_str(),
            ))
        }

        /// Format: llm.input_messages.{index}.message.contents.{content_index}.message_content.text
        pub fn content_text(index: usize, content_index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("llm.input_messages.{index}.message.contents.{content_index}.message_content.text").into_boxed_str(),
            ))
        }
    }

    /// Output messages from the LLM.
    pub mod output_messages {
        use opentelemetry::Key;

        /// Format: llm.output_messages.{index}.message.role
        pub fn role(index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("llm.output_messages.{index}.message.role").into_boxed_str(),
            ))
        }

        /// Format: llm.output_messages.{index}.message.content
        pub fn content(index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("llm.output_messages.{index}.message.content").into_boxed_str(),
            ))
        }

        /// Tool calls in output messages.
        pub mod tool_calls {
            use opentelemetry::Key;

            /// Format: llm.output_messages.{msg_index}.message.tool_calls.{call_index}.tool_call.id
            pub fn id(msg_index: usize, call_index: usize) -> Key {
                Key::from_static_str(Box::leak(
                    format!("llm.output_messages.{msg_index}.message.tool_calls.{call_index}.tool_call.id").into_boxed_str(),
                ))
            }

            /// Format: llm.output_messages.{msg_index}.message.tool_calls.{call_index}.tool_call.function.name
            pub fn function_name(msg_index: usize, call_index: usize) -> Key {
                Key::from_static_str(Box::leak(
                    format!("llm.output_messages.{msg_index}.message.tool_calls.{call_index}.tool_call.function.name").into_boxed_str(),
                ))
            }

            /// Format: llm.output_messages.{msg_index}.message.tool_calls.{call_index}.tool_call.function.arguments
            pub fn function_arguments(msg_index: usize, call_index: usize) -> Key {
                Key::from_static_str(Box::leak(
                    format!("llm.output_messages.{msg_index}.message.tool_calls.{call_index}.tool_call.function.arguments").into_boxed_str(),
                ))
            }
        }
    }

    /// Prompts for text completion (non-chat).
    pub mod prompts {
        use opentelemetry::Key;

        /// Format: llm.prompts.{index}.prompt.text
        pub fn text(index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("llm.prompts.{index}.prompt.text").into_boxed_str(),
            ))
        }
    }

    /// Choices/completions from text completion.
    pub mod choices {
        use opentelemetry::Key;

        /// Format: llm.choices.{index}.completion.text
        pub fn text(index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("llm.choices.{index}.completion.text").into_boxed_str(),
            ))
        }
    }

    /// Tools available to the LLM.
    pub mod tools {
        use opentelemetry::Key;

        /// Format: llm.tools.{index}.tool.json_schema
        pub fn json_schema(index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("llm.tools.{index}.tool.json_schema").into_boxed_str(),
            ))
        }
    }

    /// Prompt template information.
    pub mod prompt_template {
        use opentelemetry::Key;

        /// The prompt template string.
        pub const TEMPLATE: Key = Key::from_static_str("llm.prompt_template.template");

        /// JSON string of template variables.
        pub const VARIABLES: Key = Key::from_static_str("llm.prompt_template.variables");

        /// Version of the prompt template.
        pub const VERSION: Key = Key::from_static_str("llm.prompt_template.version");
    }

    /// Token count attributes.
    pub mod token_count {
        use opentelemetry::Key;

        /// Number of tokens in the prompt/input.
        pub const PROMPT: Key = Key::from_static_str("llm.token_count.prompt");

        /// Number of tokens in the completion/output.
        pub const COMPLETION: Key = Key::from_static_str("llm.token_count.completion");

        /// Total number of tokens (prompt + completion).
        pub const TOTAL: Key = Key::from_static_str("llm.token_count.total");

        /// Detailed prompt token breakdown.
        pub mod prompt_details {
            use opentelemetry::Key;

            /// Tokens read from cache.
            pub const CACHE_READ: Key =
                Key::from_static_str("llm.token_count.prompt_details.cache_read");

            /// Tokens written to cache.
            pub const CACHE_WRITE: Key =
                Key::from_static_str("llm.token_count.prompt_details.cache_write");

            /// Audio tokens in prompt.
            pub const AUDIO: Key = Key::from_static_str("llm.token_count.prompt_details.audio");
        }

        /// Detailed completion token breakdown.
        pub mod completion_details {
            use opentelemetry::Key;

            /// Reasoning tokens in completion.
            pub const REASONING: Key =
                Key::from_static_str("llm.token_count.completion_details.reasoning");

            /// Audio tokens in completion.
            pub const AUDIO: Key = Key::from_static_str("llm.token_count.completion_details.audio");
        }
    }

    /// Cost attributes.
    pub mod cost {
        use opentelemetry::Key;

        /// Cost of the prompt.
        pub const PROMPT: Key = Key::from_static_str("llm.cost.prompt");

        /// Cost of the completion.
        pub const COMPLETION: Key = Key::from_static_str("llm.cost.completion");

        /// Total cost.
        pub const TOTAL: Key = Key::from_static_str("llm.cost.total");

        /// Detailed prompt cost breakdown.
        pub mod prompt_details {
            use opentelemetry::Key;

            pub const INPUT: Key = Key::from_static_str("llm.cost.prompt_details.input");
            pub const CACHE_WRITE: Key =
                Key::from_static_str("llm.cost.prompt_details.cache_write");
            pub const CACHE_READ: Key = Key::from_static_str("llm.cost.prompt_details.cache_read");
            pub const CACHE_INPUT: Key =
                Key::from_static_str("llm.cost.prompt_details.cache_input");
            pub const AUDIO: Key = Key::from_static_str("llm.cost.prompt_details.audio");
        }

        /// Detailed completion cost breakdown.
        pub mod completion_details {
            use opentelemetry::Key;

            pub const OUTPUT: Key = Key::from_static_str("llm.cost.completion_details.output");
            pub const REASONING: Key =
                Key::from_static_str("llm.cost.completion_details.reasoning");
            pub const AUDIO: Key = Key::from_static_str("llm.cost.completion_details.audio");
        }
    }
}

// =============================================================================
// Embedding Attributes
// =============================================================================

/// Attributes for embedding spans.
pub mod embedding {
    use opentelemetry::Key;

    /// The name of the embedding model.
    pub const MODEL_NAME: Key = Key::from_static_str("embedding.model_name");

    /// The text being embedded (single embedding).
    pub const TEXT: Key = Key::from_static_str("embedding.text");

    /// The embedding vector (single embedding).
    pub const VECTOR: Key = Key::from_static_str("embedding.vector");

    /// JSON string of invocation parameters.
    pub const INVOCATION_PARAMETERS: Key = Key::from_static_str("embedding.invocation_parameters");

    /// Multiple embeddings.
    pub mod embeddings {
        use opentelemetry::Key;

        /// Format: embedding.embeddings.{index}.embedding.vector
        pub fn vector(index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("embedding.embeddings.{index}.embedding.vector").into_boxed_str(),
            ))
        }

        /// Format: embedding.embeddings.{index}.embedding.text
        pub fn text(index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("embedding.embeddings.{index}.embedding.text").into_boxed_str(),
            ))
        }
    }
}

// =============================================================================
// Tool Attributes
// =============================================================================

/// Attributes for tool spans.
pub mod tool {
    use opentelemetry::Key;

    /// Name of the tool.
    pub const NAME: Key = Key::from_static_str("tool.name");

    /// Description of the tool's purpose.
    pub const DESCRIPTION: Key = Key::from_static_str("tool.description");

    /// JSON schema of the tool.
    pub const JSON_SCHEMA: Key = Key::from_static_str("tool.json_schema");

    /// Parameters passed to the tool (JSON string).
    pub const PARAMETERS: Key = Key::from_static_str("tool.parameters");

    /// Tool ID.
    pub const ID: Key = Key::from_static_str("tool.id");
}

/// Attributes for tool calls.
pub mod tool_call {
    use opentelemetry::Key;

    /// Tool call ID.
    pub const ID: Key = Key::from_static_str("tool_call.id");

    /// Function attributes.
    pub mod function {
        use opentelemetry::Key;

        /// Function name.
        pub const NAME: Key = Key::from_static_str("tool_call.function.name");

        /// Function arguments (JSON string).
        pub const ARGUMENTS: Key = Key::from_static_str("tool_call.function.arguments");
    }
}

// =============================================================================
// Document Attributes
// =============================================================================

/// Attributes for documents (used in retrieval/reranking).
pub mod document {
    use opentelemetry::Key;

    /// Document ID.
    pub const ID: Key = Key::from_static_str("document.id");

    /// Document content.
    pub const CONTENT: Key = Key::from_static_str("document.content");

    /// Document score (relevance, similarity, etc.).
    pub const SCORE: Key = Key::from_static_str("document.score");

    /// Document metadata (JSON string).
    pub const METADATA: Key = Key::from_static_str("document.metadata");
}

// =============================================================================
// Retrieval Attributes
// =============================================================================

/// Attributes for retriever spans.
pub mod retrieval {
    /// Documents returned by retrieval.
    pub mod documents {
        use opentelemetry::Key;

        /// Format: retrieval.documents.{index}.document.id
        pub fn id(index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("retrieval.documents.{index}.document.id").into_boxed_str(),
            ))
        }

        /// Format: retrieval.documents.{index}.document.content
        pub fn content(index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("retrieval.documents.{index}.document.content").into_boxed_str(),
            ))
        }

        /// Format: retrieval.documents.{index}.document.score
        pub fn score(index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("retrieval.documents.{index}.document.score").into_boxed_str(),
            ))
        }

        /// Format: retrieval.documents.{index}.document.metadata
        pub fn metadata(index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("retrieval.documents.{index}.document.metadata").into_boxed_str(),
            ))
        }
    }
}

// =============================================================================
// Reranker Attributes
// =============================================================================

/// Attributes for reranker spans.
pub mod reranker {
    use opentelemetry::Key;

    /// The reranker model name.
    pub const MODEL_NAME: Key = Key::from_static_str("reranker.model_name");

    /// The query used for reranking.
    pub const QUERY: Key = Key::from_static_str("reranker.query");

    /// Number of top documents to return.
    pub const TOP_K: Key = Key::from_static_str("reranker.top_k");

    /// Input documents.
    pub mod input_documents {
        use opentelemetry::Key;

        pub fn id(index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("reranker.input_documents.{index}.document.id").into_boxed_str(),
            ))
        }

        pub fn content(index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("reranker.input_documents.{index}.document.content").into_boxed_str(),
            ))
        }

        pub fn score(index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("reranker.input_documents.{index}.document.score").into_boxed_str(),
            ))
        }
    }

    /// Output documents (reranked).
    pub mod output_documents {
        use opentelemetry::Key;

        pub fn id(index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("reranker.output_documents.{index}.document.id").into_boxed_str(),
            ))
        }

        pub fn content(index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("reranker.output_documents.{index}.document.content").into_boxed_str(),
            ))
        }

        pub fn score(index: usize) -> Key {
            Key::from_static_str(Box::leak(
                format!("reranker.output_documents.{index}.document.score").into_boxed_str(),
            ))
        }
    }
}

// =============================================================================
// Input/Output Attributes
// =============================================================================

/// Input attributes.
pub mod input {
    use opentelemetry::Key;

    /// The input value.
    pub const VALUE: Key = Key::from_static_str("input.value");

    /// The MIME type of the input.
    pub const MIME_TYPE: Key = Key::from_static_str("input.mime_type");
}

/// Output attributes.
pub mod output {
    use opentelemetry::Key;

    /// The output value.
    pub const VALUE: Key = Key::from_static_str("output.value");

    /// The MIME type of the output.
    pub const MIME_TYPE: Key = Key::from_static_str("output.mime_type");
}

// =============================================================================
// Session/User Attributes
// =============================================================================

/// User attributes.
pub mod user {
    use opentelemetry::Key;

    /// User ID.
    pub const ID: Key = Key::from_static_str("user.id");
}

/// Session attributes.
pub mod session {
    use opentelemetry::Key;

    /// Session ID.
    pub const ID: Key = Key::from_static_str("session.id");
}

// =============================================================================
// Exception Attributes
// =============================================================================

/// Exception attributes for error tracking.
pub mod exception {
    use opentelemetry::Key;

    /// Exception type/class name.
    pub const TYPE: Key = Key::from_static_str("exception.type");

    /// Exception message.
    pub const MESSAGE: Key = Key::from_static_str("exception.message");

    /// Exception stack trace.
    pub const STACKTRACE: Key = Key::from_static_str("exception.stacktrace");

    /// Whether the exception escaped the span.
    pub const ESCAPED: Key = Key::from_static_str("exception.escaped");
}

// =============================================================================
// Metadata Attributes
// =============================================================================

/// General metadata attribute (JSON string).
pub const METADATA: Key = Key::from_static_str("metadata");

/// Tags attributes.
pub mod tag {
    use opentelemetry::Key;

    /// List of tags.
    pub const TAGS: Key = Key::from_static_str("tag.tags");
}

// =============================================================================
// Multimodal Attributes
// =============================================================================

/// Image attributes.
pub mod image {
    use opentelemetry::Key;

    /// Image URL.
    pub const URL: Key = Key::from_static_str("image.url");
}

/// Audio attributes.
pub mod audio {
    use opentelemetry::Key;

    /// Audio URL.
    pub const URL: Key = Key::from_static_str("audio.url");

    /// Audio MIME type.
    pub const MIME_TYPE: Key = Key::from_static_str("audio.mime_type");

    /// Audio transcript.
    pub const TRANSCRIPT: Key = Key::from_static_str("audio.transcript");
}

// =============================================================================
// Agent/Graph Attributes
// =============================================================================

/// Agent attributes.
pub mod agent {
    use opentelemetry::Key;

    /// Agent name.
    pub const NAME: Key = Key::from_static_str("agent.name");
}

/// Graph node attributes.
pub mod graph {
    /// Node attributes.
    pub mod node {
        use opentelemetry::Key;

        /// Node ID.
        pub const ID: Key = Key::from_static_str("graph.node.id");

        /// Node name.
        pub const NAME: Key = Key::from_static_str("graph.node.name");

        /// Parent node ID.
        pub const PARENT_ID: Key = Key::from_static_str("graph.node.parent_id");
    }
}

// =============================================================================
// Prompt Attributes
// =============================================================================

/// Prompt management attributes.
pub mod prompt {
    use opentelemetry::Key;

    /// Prompt vendor/provider.
    pub const VENDOR: Key = Key::from_static_str("prompt.vendor");

    /// Prompt ID.
    pub const ID: Key = Key::from_static_str("prompt.id");

    /// Prompt URL.
    pub const URL: Key = Key::from_static_str("prompt.url");
}

// =============================================================================
// Message Attributes (standalone)
// =============================================================================

/// Standalone message attributes.
pub mod message {
    use opentelemetry::Key;

    /// Message role.
    pub const ROLE: Key = Key::from_static_str("message.role");

    /// Message content.
    pub const CONTENT: Key = Key::from_static_str("message.content");

    /// Function call name (deprecated, use tool_calls).
    pub const FUNCTION_CALL_NAME: Key = Key::from_static_str("message.function_call_name");

    /// Function call arguments (deprecated, use tool_calls).
    pub const FUNCTION_CALL_ARGUMENTS_JSON: Key =
        Key::from_static_str("message.function_call_arguments_json");

    /// Tool call ID (for tool responses).
    pub const TOOL_CALL_ID: Key = Key::from_static_str("message.tool_call_id");
}
