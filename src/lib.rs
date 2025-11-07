use tower_lsp::{
    Client, LanguageServer,
    jsonrpc::Result,
    lsp_types::{InitializeParams, InitializeResult, InitializedParams, MessageType},
};

pub struct Server {
    client: Client,
}

impl Server {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Server {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult::default())
    }
    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}
