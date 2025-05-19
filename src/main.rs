mod todo;

use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use todo::TodoList;
use tracing_subscriber::{self, EnvFilter};

/// MCP 待办事项服务器
/// 可以通过标准输入输出流与客户端通信
#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("启动MCP待办事项服务器...");

    // 创建TodoList服务实例
    let service = TodoList::new().serve(stdio()).await?;

    // 等待服务停止
    tracing::info!("服务已启动，等待请求...");
    service.waiting().await?;
    
    tracing::info!("服务已停止");
    Ok(())
}
