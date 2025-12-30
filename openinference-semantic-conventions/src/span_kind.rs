//! OpenInference span kinds.
//!
//! The `openinference.span.kind` attribute is required for all OpenInference spans.

use opentelemetry::Value;

/// OpenInference span kinds that identify the type of operation being traced.
///
/// See: <https://github.com/Arize-ai/openinference/blob/main/spec/traces.md>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum SpanKind {
    /// A span representing a call to a Large Language Model.
    ///
    /// For example, a call to OpenAI or Llama for chat completions or text generation.
    Llm,

    /// A span representing a call to generate embeddings.
    ///
    /// For example, a call to OpenAI to get an ada embedding for retrieval.
    Embedding,

    /// A span representing a starting point or link between LLM application steps.
    ///
    /// For example, the beginning of a request to an LLM application or glue code
    /// that passes context from a retriever to an LLM call.
    Chain,

    /// A span representing a call to an external tool.
    ///
    /// For example, a calculator, weather API, or any function execution
    /// invoked by an LLM or agent.
    Tool,

    /// A span encompassing calls to LLMs and Tools.
    ///
    /// An agent describes a reasoning block that acts on tools using the guidance of an LLM.
    Agent,

    /// A span representing a call to a vector store or database to fetch documents.
    Retriever,

    /// A span representing the reranking of input documents.
    ///
    /// For example, a cross-encoder computing relevance scores and returning top-K documents.
    Reranker,

    /// A span representing a guardrail check on inputs or outputs.
    Guardrail,

    /// A span representing an evaluation of model outputs.
    Evaluator,
}

impl SpanKind {
    /// Returns the string representation of the span kind.
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            SpanKind::Llm => "LLM",
            SpanKind::Embedding => "EMBEDDING",
            SpanKind::Chain => "CHAIN",
            SpanKind::Tool => "TOOL",
            SpanKind::Agent => "AGENT",
            SpanKind::Retriever => "RETRIEVER",
            SpanKind::Reranker => "RERANKER",
            SpanKind::Guardrail => "GUARDRAIL",
            SpanKind::Evaluator => "EVALUATOR",
        }
    }

    /// Parses a span kind from its string representation.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "LLM" => Some(SpanKind::Llm),
            "EMBEDDING" => Some(SpanKind::Embedding),
            "CHAIN" => Some(SpanKind::Chain),
            "TOOL" => Some(SpanKind::Tool),
            "AGENT" => Some(SpanKind::Agent),
            "RETRIEVER" => Some(SpanKind::Retriever),
            "RERANKER" => Some(SpanKind::Reranker),
            "GUARDRAIL" => Some(SpanKind::Guardrail),
            "EVALUATOR" => Some(SpanKind::Evaluator),
            _ => None,
        }
    }
}

impl std::fmt::Display for SpanKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<SpanKind> for Value {
    fn from(kind: SpanKind) -> Self {
        Value::String(kind.as_str().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_kind_as_str() {
        assert_eq!(SpanKind::Llm.as_str(), "LLM");
        assert_eq!(SpanKind::Embedding.as_str(), "EMBEDDING");
        assert_eq!(SpanKind::Chain.as_str(), "CHAIN");
        assert_eq!(SpanKind::Tool.as_str(), "TOOL");
        assert_eq!(SpanKind::Agent.as_str(), "AGENT");
        assert_eq!(SpanKind::Retriever.as_str(), "RETRIEVER");
        assert_eq!(SpanKind::Reranker.as_str(), "RERANKER");
        assert_eq!(SpanKind::Guardrail.as_str(), "GUARDRAIL");
        assert_eq!(SpanKind::Evaluator.as_str(), "EVALUATOR");
    }

    #[test]
    fn test_span_kind_from_str() {
        assert_eq!(SpanKind::from_str("LLM"), Some(SpanKind::Llm));
        assert_eq!(SpanKind::from_str("llm"), Some(SpanKind::Llm));
        assert_eq!(SpanKind::from_str("Llm"), Some(SpanKind::Llm));
        assert_eq!(SpanKind::from_str("invalid"), None);
    }

    #[test]
    fn test_span_kind_display() {
        assert_eq!(format!("{}", SpanKind::Llm), "LLM");
        assert_eq!(format!("{}", SpanKind::Agent), "AGENT");
    }
}
