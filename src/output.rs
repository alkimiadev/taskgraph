//! Output formatting utilities.

/// Output format options.
#[derive(Debug, Clone, Copy, Default)]
pub enum OutputFormat {
    /// Plain text (default).
    #[default]
    Plain,
    /// JSON format.
    Json,
    /// DOT format (for graphviz).
    Dot,
}

/// Format output according to the specified format.
pub fn format_output<T: serde::Serialize>(
    value: &T,
    format: OutputFormat,
) -> Result<String, serde_json::Error> {
    match format {
        OutputFormat::Plain => {
            // Default to JSON for now, will be customized per command
            serde_json::to_string_pretty(value)
        }
        OutputFormat::Json => serde_json::to_string_pretty(value),
        OutputFormat::Dot => {
            // DOT format is handled specially by graph commands
            serde_json::to_string_pretty(value)
        }
    }
}
