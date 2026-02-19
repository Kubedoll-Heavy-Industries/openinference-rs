//! TraceConfig for controlling OpenInference privacy and observability settings.
//!
//! Implements the [OpenInference Configuration spec](https://github.com/Arize-ai/openinference/blob/main/spec/configuration.md),
//! supporting environment variable loading, programmatic builder construction, and
//! compound hide logic (e.g., `hide_inputs` implies hiding input messages, text, and images).

use std::env;

/// Placeholder value used when content is redacted due to privacy configuration.
pub const REDACTED: &str = "__REDACTED__";

// Environment variable names per the OpenInference Configuration spec.
const ENV_HIDE_INPUTS: &str = "OPENINFERENCE_HIDE_INPUTS";
const ENV_HIDE_OUTPUTS: &str = "OPENINFERENCE_HIDE_OUTPUTS";
const ENV_HIDE_INPUT_MESSAGES: &str = "OPENINFERENCE_HIDE_INPUT_MESSAGES";
const ENV_HIDE_OUTPUT_MESSAGES: &str = "OPENINFERENCE_HIDE_OUTPUT_MESSAGES";
const ENV_HIDE_INPUT_IMAGES: &str = "OPENINFERENCE_HIDE_INPUT_IMAGES";
const ENV_HIDE_INPUT_TEXT: &str = "OPENINFERENCE_HIDE_INPUT_TEXT";
const ENV_HIDE_OUTPUT_TEXT: &str = "OPENINFERENCE_HIDE_OUTPUT_TEXT";
const ENV_HIDE_LLM_INVOCATION_PARAMETERS: &str = "OPENINFERENCE_HIDE_LLM_INVOCATION_PARAMETERS";
const ENV_HIDE_EMBEDDING_VECTORS: &str = "OPENINFERENCE_HIDE_EMBEDDING_VECTORS";
const ENV_HIDE_EMBEDDINGS_VECTORS: &str = "OPENINFERENCE_HIDE_EMBEDDINGS_VECTORS";
const ENV_HIDE_EMBEDDINGS_TEXT: &str = "OPENINFERENCE_HIDE_EMBEDDINGS_TEXT";
const ENV_HIDE_PROMPTS: &str = "OPENINFERENCE_HIDE_PROMPTS";
const ENV_HIDE_CHOICES: &str = "OPENINFERENCE_HIDE_CHOICES";
const ENV_BASE64_IMAGE_MAX_LENGTH: &str = "OPENINFERENCE_BASE64_IMAGE_MAX_LENGTH";

const DEFAULT_BASE64_IMAGE_MAX_LENGTH: usize = 32_000;

/// Controls the observability level of OpenInference tracing.
///
/// `TraceConfig` lets you hide sensitive information from being recorded in spans
/// and limit the size of base64-encoded images. Values can be set programmatically
/// via the builder, read from environment variables with [`TraceConfig::from_env`],
/// or left at their defaults (maximum observability).
///
/// Precedence: builder values > environment variables > defaults.
///
/// # Example
///
/// ```
/// use openinference_instrumentation::TraceConfig;
///
/// // From environment variables (falls back to defaults)
/// let config = TraceConfig::from_env();
///
/// // Programmatic builder (overrides env vars)
/// let config = TraceConfig::builder()
///     .hide_inputs(true)
///     .hide_outputs(true)
///     .base64_image_max_length(16_000)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct TraceConfig {
    pub hide_inputs: bool,
    pub hide_outputs: bool,
    pub hide_input_messages: bool,
    pub hide_output_messages: bool,
    pub hide_input_images: bool,
    pub hide_input_text: bool,
    pub hide_output_text: bool,
    pub hide_llm_invocation_parameters: bool,
    /// Deprecated: use `hide_embeddings_vectors` instead.
    pub hide_embedding_vectors: bool,
    pub hide_embeddings_vectors: bool,
    pub hide_embeddings_text: bool,
    pub hide_prompts: bool,
    pub hide_choices: bool,
    pub base64_image_max_length: usize,
    /// Whether to also emit OTel GenAI semantic convention attributes.
    /// Carried forward from the original SpanConfig.
    pub emit_gen_ai_attributes: bool,
}

impl Default for TraceConfig {
    /// Returns a config with maximum observability: nothing hidden.
    fn default() -> Self {
        Self {
            hide_inputs: false,
            hide_outputs: false,
            hide_input_messages: false,
            hide_output_messages: false,
            hide_input_images: false,
            hide_input_text: false,
            hide_output_text: false,
            hide_llm_invocation_parameters: false,
            hide_embedding_vectors: false,
            hide_embeddings_vectors: false,
            hide_embeddings_text: false,
            hide_prompts: false,
            hide_choices: false,
            base64_image_max_length: DEFAULT_BASE64_IMAGE_MAX_LENGTH,
            emit_gen_ai_attributes: true,
        }
    }
}

impl TraceConfig {
    /// Create a [`TraceConfigBuilder`] for programmatic construction.
    pub fn builder() -> TraceConfigBuilder {
        TraceConfigBuilder::default()
    }

    /// Load configuration from environment variables, falling back to defaults.
    ///
    /// Boolean env vars accept `true`/`false` and `1`/`0` (case-insensitive).
    /// Invalid values are silently ignored and the default is used.
    pub fn from_env() -> Self {
        Self {
            hide_inputs: parse_bool_env(ENV_HIDE_INPUTS, false),
            hide_outputs: parse_bool_env(ENV_HIDE_OUTPUTS, false),
            hide_input_messages: parse_bool_env(ENV_HIDE_INPUT_MESSAGES, false),
            hide_output_messages: parse_bool_env(ENV_HIDE_OUTPUT_MESSAGES, false),
            hide_input_images: parse_bool_env(ENV_HIDE_INPUT_IMAGES, false),
            hide_input_text: parse_bool_env(ENV_HIDE_INPUT_TEXT, false),
            hide_output_text: parse_bool_env(ENV_HIDE_OUTPUT_TEXT, false),
            hide_llm_invocation_parameters: parse_bool_env(
                ENV_HIDE_LLM_INVOCATION_PARAMETERS,
                false,
            ),
            hide_embedding_vectors: parse_bool_env(ENV_HIDE_EMBEDDING_VECTORS, false),
            hide_embeddings_vectors: parse_bool_env(ENV_HIDE_EMBEDDINGS_VECTORS, false),
            hide_embeddings_text: parse_bool_env(ENV_HIDE_EMBEDDINGS_TEXT, false),
            hide_prompts: parse_bool_env(ENV_HIDE_PROMPTS, false),
            hide_choices: parse_bool_env(ENV_HIDE_CHOICES, false),
            base64_image_max_length: parse_usize_env(
                ENV_BASE64_IMAGE_MAX_LENGTH,
                DEFAULT_BASE64_IMAGE_MAX_LENGTH,
            ),
            emit_gen_ai_attributes: true,
        }
    }

    // -- Compound hide helpers ------------------------------------------------
    // These reflect the spec's cascading logic: e.g., hiding all inputs
    // implies hiding input messages, input text, and input images.

    /// Whether input messages should be hidden.
    ///
    /// True if `hide_inputs` or `hide_input_messages` is set.
    pub fn should_hide_input_messages(&self) -> bool {
        self.hide_inputs || self.hide_input_messages
    }

    /// Whether output messages should be hidden.
    ///
    /// True if `hide_outputs` or `hide_output_messages` is set.
    pub fn should_hide_output_messages(&self) -> bool {
        self.hide_outputs || self.hide_output_messages
    }

    /// Whether input text should be hidden.
    ///
    /// True if `hide_inputs`, `hide_input_messages`, or `hide_input_text` is set.
    pub fn should_hide_input_text(&self) -> bool {
        self.hide_inputs || self.hide_input_messages || self.hide_input_text
    }

    /// Whether output text should be hidden.
    ///
    /// True if `hide_outputs`, `hide_output_messages`, or `hide_output_text` is set.
    pub fn should_hide_output_text(&self) -> bool {
        self.hide_outputs || self.hide_output_messages || self.hide_output_text
    }

    /// Whether input images should be hidden.
    ///
    /// True if `hide_inputs`, `hide_input_messages`, or `hide_input_images` is set.
    pub fn should_hide_input_images(&self) -> bool {
        self.hide_inputs || self.hide_input_messages || self.hide_input_images
    }

    /// Whether embedding vectors should be hidden.
    ///
    /// True if either the deprecated `hide_embedding_vectors` or
    /// `hide_embeddings_vectors` is set.
    pub fn should_hide_embedding_vectors(&self) -> bool {
        self.hide_embedding_vectors || self.hide_embeddings_vectors
    }

    /// Whether prompts should be hidden (completions API).
    ///
    /// True if `hide_inputs` or `hide_prompts` is set.
    pub fn should_hide_prompts(&self) -> bool {
        self.hide_inputs || self.hide_prompts
    }

    /// Whether choices should be hidden (completions API outputs).
    ///
    /// True if `hide_outputs` or `hide_choices` is set.
    pub fn should_hide_choices(&self) -> bool {
        self.hide_outputs || self.hide_choices
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`TraceConfig`] with programmatic overrides.
///
/// Any field not explicitly set will fall back to the environment variable,
/// then to the default value.
#[derive(Debug, Default)]
pub struct TraceConfigBuilder {
    hide_inputs: Option<bool>,
    hide_outputs: Option<bool>,
    hide_input_messages: Option<bool>,
    hide_output_messages: Option<bool>,
    hide_input_images: Option<bool>,
    hide_input_text: Option<bool>,
    hide_output_text: Option<bool>,
    hide_llm_invocation_parameters: Option<bool>,
    hide_embedding_vectors: Option<bool>,
    hide_embeddings_vectors: Option<bool>,
    hide_embeddings_text: Option<bool>,
    hide_prompts: Option<bool>,
    hide_choices: Option<bool>,
    base64_image_max_length: Option<usize>,
    emit_gen_ai_attributes: Option<bool>,
}

macro_rules! builder_setter {
    ($name:ident, bool) => {
        pub fn $name(mut self, value: bool) -> Self {
            self.$name = Some(value);
            self
        }
    };
    ($name:ident, usize) => {
        pub fn $name(mut self, value: usize) -> Self {
            self.$name = Some(value);
            self
        }
    };
}

impl TraceConfigBuilder {
    builder_setter!(hide_inputs, bool);
    builder_setter!(hide_outputs, bool);
    builder_setter!(hide_input_messages, bool);
    builder_setter!(hide_output_messages, bool);
    builder_setter!(hide_input_images, bool);
    builder_setter!(hide_input_text, bool);
    builder_setter!(hide_output_text, bool);
    builder_setter!(hide_llm_invocation_parameters, bool);
    builder_setter!(hide_embedding_vectors, bool);
    builder_setter!(hide_embeddings_vectors, bool);
    builder_setter!(hide_embeddings_text, bool);
    builder_setter!(hide_prompts, bool);
    builder_setter!(hide_choices, bool);
    builder_setter!(base64_image_max_length, usize);
    builder_setter!(emit_gen_ai_attributes, bool);

    /// Build the [`TraceConfig`].
    ///
    /// Fields set on the builder take precedence over env vars, which take
    /// precedence over defaults.
    pub fn build(self) -> TraceConfig {
        let env = TraceConfig::from_env();
        TraceConfig {
            hide_inputs: self.hide_inputs.unwrap_or(env.hide_inputs),
            hide_outputs: self.hide_outputs.unwrap_or(env.hide_outputs),
            hide_input_messages: self.hide_input_messages.unwrap_or(env.hide_input_messages),
            hide_output_messages: self.hide_output_messages.unwrap_or(env.hide_output_messages),
            hide_input_images: self.hide_input_images.unwrap_or(env.hide_input_images),
            hide_input_text: self.hide_input_text.unwrap_or(env.hide_input_text),
            hide_output_text: self.hide_output_text.unwrap_or(env.hide_output_text),
            hide_llm_invocation_parameters: self
                .hide_llm_invocation_parameters
                .unwrap_or(env.hide_llm_invocation_parameters),
            hide_embedding_vectors: self
                .hide_embedding_vectors
                .unwrap_or(env.hide_embedding_vectors),
            hide_embeddings_vectors: self
                .hide_embeddings_vectors
                .unwrap_or(env.hide_embeddings_vectors),
            hide_embeddings_text: self.hide_embeddings_text.unwrap_or(env.hide_embeddings_text),
            hide_prompts: self.hide_prompts.unwrap_or(env.hide_prompts),
            hide_choices: self.hide_choices.unwrap_or(env.hide_choices),
            base64_image_max_length: self
                .base64_image_max_length
                .unwrap_or(env.base64_image_max_length),
            emit_gen_ai_attributes: self
                .emit_gen_ai_attributes
                .unwrap_or(env.emit_gen_ai_attributes),
        }
    }
}

// =============================================================================
// Env parsing helpers
// =============================================================================

fn parse_bool_env(key: &str, default: bool) -> bool {
    match env::var(key) {
        Ok(val) => match val.to_lowercase().as_str() {
            "true" | "1" => true,
            "false" | "0" => false,
            _ => default,
        },
        Err(_) => default,
    }
}

fn parse_usize_env(key: &str, default: usize) -> usize {
    match env::var(key) {
        Ok(val) => val.parse().unwrap_or(default),
        Err(_) => default,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Env-var tests mutate process-wide state, so serialize them.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_default_is_all_visible() {
        let config = TraceConfig::default();
        assert!(!config.hide_inputs);
        assert!(!config.hide_outputs);
        assert!(!config.hide_input_messages);
        assert!(!config.hide_output_messages);
        assert!(!config.hide_input_images);
        assert!(!config.hide_input_text);
        assert!(!config.hide_output_text);
        assert!(!config.hide_llm_invocation_parameters);
        assert!(!config.hide_embedding_vectors);
        assert!(!config.hide_embeddings_vectors);
        assert!(!config.hide_embeddings_text);
        assert!(!config.hide_prompts);
        assert!(!config.hide_choices);
        assert_eq!(config.base64_image_max_length, 32_000);
        assert!(config.emit_gen_ai_attributes);
    }

    #[test]
    fn test_from_env_reads_booleans() {
        let _lock = ENV_LOCK.lock().unwrap();

        env::set_var(ENV_HIDE_INPUTS, "true");
        env::set_var(ENV_HIDE_OUTPUTS, "1");
        env::set_var(ENV_HIDE_INPUT_MESSAGES, "TRUE");
        env::set_var(ENV_HIDE_OUTPUT_MESSAGES, "false");
        env::set_var(ENV_HIDE_INPUT_IMAGES, "0");
        env::set_var(ENV_BASE64_IMAGE_MAX_LENGTH, "16000");

        let config = TraceConfig::from_env();

        assert!(config.hide_inputs);
        assert!(config.hide_outputs);
        assert!(config.hide_input_messages);
        assert!(!config.hide_output_messages);
        assert!(!config.hide_input_images);
        assert_eq!(config.base64_image_max_length, 16_000);

        // Clean up
        env::remove_var(ENV_HIDE_INPUTS);
        env::remove_var(ENV_HIDE_OUTPUTS);
        env::remove_var(ENV_HIDE_INPUT_MESSAGES);
        env::remove_var(ENV_HIDE_OUTPUT_MESSAGES);
        env::remove_var(ENV_HIDE_INPUT_IMAGES);
        env::remove_var(ENV_BASE64_IMAGE_MAX_LENGTH);
    }

    #[test]
    fn test_from_env_invalid_values_use_defaults() {
        let _lock = ENV_LOCK.lock().unwrap();

        env::set_var(ENV_HIDE_INPUTS, "not_a_bool");
        env::set_var(ENV_BASE64_IMAGE_MAX_LENGTH, "not_a_number");

        let config = TraceConfig::from_env();

        assert!(!config.hide_inputs);
        assert_eq!(config.base64_image_max_length, 32_000);

        env::remove_var(ENV_HIDE_INPUTS);
        env::remove_var(ENV_BASE64_IMAGE_MAX_LENGTH);
    }

    #[test]
    fn test_builder_overrides_env() {
        let _lock = ENV_LOCK.lock().unwrap();

        env::set_var(ENV_HIDE_INPUTS, "true");

        let config = TraceConfig::builder().hide_inputs(false).build();

        assert!(!config.hide_inputs);

        env::remove_var(ENV_HIDE_INPUTS);
    }

    #[test]
    fn test_builder_falls_through_to_env() {
        let _lock = ENV_LOCK.lock().unwrap();

        env::set_var(ENV_HIDE_OUTPUTS, "true");

        let config = TraceConfig::builder().hide_inputs(true).build();

        assert!(config.hide_inputs);
        assert!(config.hide_outputs); // from env

        env::remove_var(ENV_HIDE_OUTPUTS);
    }

    #[test]
    fn test_compound_hide_inputs_implies_messages_text_images() {
        let _lock = ENV_LOCK.lock().unwrap();
        let config = TraceConfig::builder().hide_inputs(true).build();

        assert!(config.should_hide_input_messages());
        assert!(config.should_hide_input_text());
        assert!(config.should_hide_input_images());
        assert!(config.should_hide_prompts());
    }

    #[test]
    fn test_compound_hide_outputs_implies_messages_text_choices() {
        let _lock = ENV_LOCK.lock().unwrap();
        let config = TraceConfig::builder().hide_outputs(true).build();

        assert!(config.should_hide_output_messages());
        assert!(config.should_hide_output_text());
        assert!(config.should_hide_choices());
    }

    #[test]
    fn test_compound_hide_input_messages_implies_text_and_images() {
        let _lock = ENV_LOCK.lock().unwrap();
        let config = TraceConfig::builder().hide_input_messages(true).build();

        assert!(config.should_hide_input_messages());
        assert!(config.should_hide_input_text());
        assert!(config.should_hide_input_images());
        // But not prompts (those are only hidden by hide_inputs or hide_prompts)
        assert!(!config.should_hide_prompts());
    }

    #[test]
    fn test_compound_hide_output_messages_implies_text() {
        let _lock = ENV_LOCK.lock().unwrap();
        let config = TraceConfig::builder().hide_output_messages(true).build();

        assert!(config.should_hide_output_messages());
        assert!(config.should_hide_output_text());
        // But not choices
        assert!(!config.should_hide_choices());
    }

    #[test]
    fn test_deprecated_hide_embedding_vectors() {
        let _lock = ENV_LOCK.lock().unwrap();
        let config = TraceConfig::builder().hide_embedding_vectors(true).build();
        assert!(config.should_hide_embedding_vectors());

        let config2 = TraceConfig::builder()
            .hide_embeddings_vectors(true)
            .build();
        assert!(config2.should_hide_embedding_vectors());
    }

    #[test]
    fn test_redacted_constant() {
        assert_eq!(REDACTED, "__REDACTED__");
    }

    #[test]
    fn test_builder_emit_gen_ai_attributes() {
        let _lock = ENV_LOCK.lock().unwrap();
        let config = TraceConfig::builder()
            .emit_gen_ai_attributes(false)
            .build();
        assert!(!config.emit_gen_ai_attributes);
    }
}
