use std::collections::HashMap;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

mod codelens;

#[derive(Debug)]
struct Backend {
    client: Client,
    document_map: RwLock<HashMap<Url, String>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                code_lens_provider: Some(CodeLensOptions {
                    resolve_provider: Some(false),
                }),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["zed-restclient::send_request".to_string()],
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Zed REST Client Sidecar initialized.")
            .await;
    }

    async fn execute_command(
        &self,
        params: ExecuteCommandParams,
    ) -> Result<Option<serde_json::Value>> {
        self.client
            .log_message(
                MessageType::INFO,
                format!(
                    "Executing command: {} with args: {:?}",
                    params.command, params.arguments
                ),
            )
            .await;
        // Später werden wir hier reqwest einbauen und die echte Anfrage verschicken!
        Ok(None)
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;

        self.document_map.write().await.insert(uri.clone(), text);

        self.client
            .log_message(MessageType::INFO, format!("Opened file: {}", uri))
            .await;
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.pop() {
            self.document_map.write().await.insert(uri, change.text);
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.document_map
            .write()
            .await
            .remove(&params.text_document.uri);
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let uri = params.text_document.uri;
        let mut actions = Vec::new();

        if let Some(text) = self.document_map.read().await.get(&uri) {
            let start_lines = codelens::find_request_starts(text);
            
            // Check if the cursor is near any request start
            let cursor_line = params.range.start.line as usize;
            
            // Allow the lightbulb to appear if the cursor is within the request block (roughly 20 lines max, or just any block)
            if start_lines.iter().any(|marker| cursor_line >= marker.display_line && cursor_line <= marker.display_line + 20) {
                // Find the specific request block the user is in
                if let Some(closest_marker) = start_lines.iter().filter(|m| m.display_line <= cursor_line).last() {
                    actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                        title: "▶ Send Request".to_string(),
                        kind: Some(CodeActionKind::new("source")),
                        command: Some(Command {
                            title: "▶ Send Request".to_string(),
                            command: "zed-restclient::send_request".to_string(),
                            arguments: Some(vec![serde_json::Value::Number(
                                serde_json::Number::from(closest_marker.block_index),
                            )]),
                        }),
                        ..Default::default()
                    }));
                }
            }
        }
        
        Ok(Some(actions))
    }

    async fn code_lens(&self, params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        let uri = params.text_document.uri;

        let mut lenses = Vec::new();

        if let Some(text) = self.document_map.read().await.get(&uri) {
            let start_lines = codelens::find_request_starts(text);

            for marker in start_lines {
                let position_start = Position {
                    line: marker.display_line as u32,
                    character: 0,
                };
                // Make the range span to character 100 so Zed realizes it covers text
                let position_end = Position {
                    line: marker.display_line as u32,
                    character: 100,
                };

                lenses.push(CodeLens {
                    range: Range {
                        start: position_start,
                        end: position_end,
                    },
                    command: Some(Command {
                        title: "▶ Send Request".to_string(),
                        command: "zed-restclient::send_request".to_string(),
                        arguments: Some(vec![serde_json::Value::Number(serde_json::Number::from(
                            marker.block_index,
                        ))]),
                    }),
                    data: None,
                });
            }
        }

        Ok(Some(lenses))
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        document_map: RwLock::new(HashMap::new()),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
