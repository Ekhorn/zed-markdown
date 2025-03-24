use zed_extension_api::{
    self as zed, lsp::CompletionKind, serde_json, CodeLabel, CodeLabelSpan, LanguageServerId,
};

#[derive(Default)]
struct MarkdownExtension {}

impl zed::Extension for MarkdownExtension {
    fn new() -> Self {
        Self::default()
    }
}

zed::register_extension!(MarkdownExtension);
