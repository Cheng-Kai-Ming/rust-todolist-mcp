use std::sync::Arc;

use chrono::{DateTime, Utc};
use rmcp::{
    Error as McpError, RoleServer, ServerHandler, model::*, 
    service::RequestContext, tool,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex;
use uuid::Uuid;

/// Todo项结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建新Todo的请求参数
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
}

/// 更新Todo的请求参数
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdateTodoRequest {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

/// TodoList服务
#[derive(Clone)]
pub struct TodoList {
    todos: Arc<Mutex<Vec<TodoItem>>>,
}

#[tool(tool_box)]
impl TodoList {
    pub fn new() -> Self {
        Self {
            todos: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 列出所有待办事项
    #[tool(description = "列出所有的待办事项")]
    async fn list_todos(&self) -> Result<CallToolResult, McpError> {
        let todos = self.todos.lock().await;
        let todos_json = serde_json::to_string_pretty(&*todos)
            .map_err(|e| McpError::internal_error("序列化失败", Some(json!({"error": e.to_string()}))))?;
        
        Ok(CallToolResult::success(vec![Content::text(todos_json)]))
    }

    /// 创建一个新的待办事项
    #[tool(description = "创建一个新的待办事项")]
    async fn create_todo(
        &self,
        #[tool(aggr)] req: CreateTodoRequest,
    ) -> Result<CallToolResult, McpError> {
        let now = Utc::now();
        let todo = TodoItem {
            id: Uuid::new_v4().to_string(),
            title: req.title,
            description: req.description,
            completed: false,
            created_at: now,
            updated_at: now,
        };

        let mut todos = self.todos.lock().await;
        todos.push(todo.clone());

        let todo_json = serde_json::to_string_pretty(&todo)
            .map_err(|e| McpError::internal_error("序列化失败", Some(json!({"error": e.to_string()}))))?;
        
        Ok(CallToolResult::success(vec![Content::text(todo_json)]))
    }

    /// 更新待办事项
    #[tool(description = "更新待办事项")]
    async fn update_todo(
        &self,
        #[tool(aggr)] req: UpdateTodoRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut todos = self.todos.lock().await;
        let todo = todos.iter_mut().find(|t| t.id == req.id);
        
        match todo {
            Some(todo) => {
                if let Some(title) = req.title {
                    todo.title = title;
                }
                if let Some(description) = req.description {
                    todo.description = Some(description);
                }
                if let Some(completed) = req.completed {
                    todo.completed = completed;
                }
                todo.updated_at = Utc::now();

                let todo_json = serde_json::to_string_pretty(&todo)
                    .map_err(|e| McpError::internal_error("序列化失败", Some(json!({"error": e.to_string()}))))?;
                
                Ok(CallToolResult::success(vec![Content::text(todo_json)]))
            },
            None => Err(McpError::invalid_params(
                "找不到指定ID的待办事项",
                Some(json!({"id": req.id})),
            )),
        }
    }

    /// 删除待办事项
    #[tool(description = "删除待办事项")]
    async fn delete_todo(
        &self,
        #[tool(param)]
        #[schemars(description = "待办事项ID")]
        id: String,
    ) -> Result<CallToolResult, McpError> {
        let mut todos = self.todos.lock().await;
        let index = todos.iter().position(|t| t.id == id);
        
        match index {
            Some(idx) => {
                todos.remove(idx);
                Ok(CallToolResult::success(vec![Content::text(
                    format!("成功删除ID为 {} 的待办事项", id)
                )]))
            },
            None => Err(McpError::invalid_params(
                "找不到指定ID的待办事项",
                Some(json!({"id": id})),
            )),
        }
    }

    /// 获取单个待办事项详情
    #[tool(description = "获取单个待办事项详情")]
    async fn get_todo(
        &self,
        #[tool(param)]
        #[schemars(description = "待办事项ID")]
        id: String,
    ) -> Result<CallToolResult, McpError> {
        let todos = self.todos.lock().await;
        let todo = todos.iter().find(|t| t.id == id);
        
        match todo {
            Some(todo) => {
                let todo_json = serde_json::to_string_pretty(todo)
                    .map_err(|e| McpError::internal_error("序列化失败", Some(json!({"error": e.to_string()}))))?;
                
                Ok(CallToolResult::success(vec![Content::text(todo_json)]))
            },
            None => Err(McpError::invalid_params(
                "找不到指定ID的待办事项",
                Some(json!({"id": id})),
            )),
        }
    }

    /// 标记待办事项为已完成
    #[tool(description = "标记待办事项为已完成")]
    async fn complete_todo(
        &self,
        #[tool(param)]
        #[schemars(description = "待办事项ID")]
        id: String,
    ) -> Result<CallToolResult, McpError> {
        let mut todos = self.todos.lock().await;
        let todo = todos.iter_mut().find(|t| t.id == id);
        
        match todo {
            Some(todo) => {
                todo.completed = true;
                todo.updated_at = Utc::now();

                let todo_json = serde_json::to_string_pretty(&todo)
                    .map_err(|e| McpError::internal_error("序列化失败", Some(json!({"error": e.to_string()}))))?;
                
                Ok(CallToolResult::success(vec![Content::text(todo_json)]))
            },
            None => Err(McpError::invalid_params(
                "找不到指定ID的待办事项",
                Some(json!({"id": id})),
            )),
        }
    }
}

#[tool(tool_box)]
impl ServerHandler for TodoList {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("这是一个待办事项服务器，您可以使用它来管理您的待办事项列表。使用list_todos查看所有待办事项，create_todo创建新的待办事项，update_todo更新现有待办事项，delete_todo删除待办事项，get_todo获取待办事项详情，complete_todo将待办事项标记为已完成。".to_string()),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        Ok(self.get_info())
    }
} 
