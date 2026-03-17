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
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
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

    async fn code_lens(&self, params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        let uri = params.text_document.uri;

        let mut lenses = Vec::new();

        if let Some(text) = self.document_map.read().await.get(&uri) {
            let start_lines = codelens::find_request_starts(text);

            for line_idx in start_lines {
                let position = Position {
                    line: line_idx as u32,
                    character: 0,
                };

                lenses.push(CodeLens {
                    range: Range {
                        start: position,
                        end: position,
                    },
                    command: Some(Command {
                        title: "▶ Send Request".to_string(),
                        command: "zed-restclient::send_request".to_string(),
                        arguments: Some(vec![serde_json::Value::Number(serde_json::Number::from(
                            line_idx,
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
