use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
struct HighlightPattern {
    pattern: String,
    case_sensitive: bool,
    whole_word: bool,
    is_regex: bool,
    color_index: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "command")]
enum HighlightCommand {
    #[serde(rename = "add")]
    Add { pattern: HighlightPattern },
    #[serde(rename = "remove")]
    Remove { pattern: String },
    #[serde(rename = "clear")]
    Clear,
}

struct Backend {
    client: Client,
    patterns: Arc<RwLock<Vec<HighlightPattern>>>,
    documents: Arc<RwLock<HashMap<String, String>>>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            patterns: Arc::new(RwLock::new(Vec::new())),
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn find_matches(&self, text: &str) -> Vec<SemanticToken> {
        let patterns = self.patterns.read().await;
        let mut tokens = Vec::new();
        
        for (pattern_idx, pattern) in patterns.iter().enumerate() {
            let regex_pattern = if pattern.is_regex {
                pattern.pattern.clone()
            } else if pattern.whole_word {
                format!(r"\b{}\b", regex::escape(&pattern.pattern))
            } else {
                regex::escape(&pattern.pattern)
            };

            let regex = match if pattern.case_sensitive {
                Regex::new(&regex_pattern)
            } else {
                Regex::new(&format!("(?i){}", regex_pattern))
            } {
                Ok(r) => r,
                Err(_) => continue,
            };

            let lines: Vec<&str> = text.lines().collect();
            for (line_idx, line) in lines.iter().enumerate() {
                for mat in regex.find_iter(line) {
                    tokens.push(SemanticToken {
                        delta_line: if tokens.is_empty() { line_idx as u32 } else { 0 },
                        delta_start: mat.start() as u32,
                        length: (mat.end() - mat.start()) as u32,
                        token_type: (pattern_idx % 8) as u32, // Cycle through 8 token types
                        token_modifiers_bitset: 0,
                    });
                }
            }
        }

        // Sort tokens by position
        tokens.sort_by(|a, b| {
            a.delta_line.cmp(&b.delta_line)
                .then(a.delta_start.cmp(&b.delta_start))
        });

        // Convert to delta encoding
        let mut prev_line = 0;
        let mut prev_start = 0;
        
        for token in &mut tokens {
            let line = prev_line + token.delta_line;
            let start = if token.delta_line == 0 {
                prev_start + token.delta_start
            } else {
                token.delta_start
            };
            
            token.delta_line = line - prev_line;
            token.delta_start = start - if token.delta_line == 0 { prev_start } else { 0 };
            
            prev_line = line;
            prev_start = start;
        }

        tokens
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            work_done_progress_options: WorkDoneProgressOptions::default(),
                            legend: SemanticTokensLegend {
                                token_types: vec![
                                    SemanticTokenType::KEYWORD,
                                    SemanticTokenType::STRING,
                                    SemanticTokenType::NUMBER,
                                    SemanticTokenType::COMMENT,
                                    SemanticTokenType::OPERATOR,
                                    SemanticTokenType::VARIABLE,
                                    SemanticTokenType::FUNCTION,
                                    SemanticTokenType::CLASS,
                                ],
                                token_modifiers: vec![],
                            },
                            range: Some(true),
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                        },
                    ),
                ),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec![
                        "highlight.add".to_string(),
                        "highlight.remove".to_string(),
                        "highlight.clear".to_string(),
                    ],
                    work_done_progress_options: WorkDoneProgressOptions::default(),
                }),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Highlight LSP server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let mut documents = self.documents.write().await;
        documents.insert(params.text_document.uri.to_string(), params.text_document.text);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let mut documents = self.documents.write().await;
        if let Some(change) = params.content_changes.into_iter().next() {
            documents.insert(params.text_document.uri.to_string(), change.text);
        }
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let documents = self.documents.read().await;
        if let Some(text) = documents.get(&params.text_document.uri.to_string()) {
            let tokens = self.find_matches(text).await;
            return Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
                result_id: None,
                data: tokens,
            })));
        }
        Ok(None)
    }

    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<serde_json::Value>> {
        match params.command.as_str() {
            "highlight.add" => {
                if let Some(arg) = params.arguments.get(0) {
                    if let Ok(pattern) = serde_json::from_value::<HighlightPattern>(arg.clone()) {
                        let mut patterns = self.patterns.write().await;
                        patterns.push(pattern);
                        self.client
                            .log_message(MessageType::INFO, "Pattern added")
                            .await;
                    }
                }
            }
            "highlight.remove" => {
                if let Some(arg) = params.arguments.get(0) {
                    if let Ok(pattern_text) = serde_json::from_value::<String>(arg.clone()) {
                        let mut patterns = self.patterns.write().await;
                        patterns.retain(|p| p.pattern != pattern_text);
                        self.client
                            .log_message(MessageType::INFO, "Pattern removed")
                            .await;
                    }
                }
            }
            "highlight.clear" => {
                let mut patterns = self.patterns.write().await;
                patterns.clear();
                self.client
                    .log_message(MessageType::INFO, "All patterns cleared")
                    .await;
            }
            _ => {}
        }

        // Refresh semantic tokens for all open documents
        self.client
            .semantic_tokens_refresh()
            .await
            .unwrap_or_default();

        Ok(None)
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}