mod todo;

use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use todo::TodoList;
use tracing_subscriber::{self, EnvFilter};

/// MCP Todo Server
/// Communicates with clients through standard input/output streams
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting MCP Todo Server...");

    // Create TodoList service instance
    let service = TodoList::new().serve(stdio()).await?;

    // Wait for service to stop
    tracing::info!("Service started, waiting for requests...");
    service.waiting().await?;
    
    tracing::info!("Service stopped");
    Ok(())
}
