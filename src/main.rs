use blade_analyzer::Server;
use tokio;
use tower_lsp::{Client, LspService, Server as LspServer};

fn init_server(client: Client) -> Server {
    Server::new(client)
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(init_server);
    LspServer::new(stdin, stdout, socket).serve(service).await;
}
