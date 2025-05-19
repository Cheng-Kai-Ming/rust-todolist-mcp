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

/// Todo item structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request parameters for creating a new Todo
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
}

/// Request parameters for updating a Todo
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdateTodoRequest {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

/// TodoList service
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

    /// List all todo items
    #[tool(description = "List all todo items")]
    async fn list_todos(&self) -> Result<CallToolResult, McpError> {
        let todos = self.todos.lock().await;
        let todos_json = serde_json::to_string_pretty(&*todos)
            .map_err(|e| McpError::internal_error("Serialization failed", Some(json!({"error": e.to_string()}))))?;
        
        Ok(CallToolResult::success(vec![Content::text(todos_json)]))
    }

    /// Create a new todo item
    #[tool(description = "Create a new todo item")]
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
            .map_err(|e| McpError::internal_error("Serialization failed", Some(json!({"error": e.to_string()}))))?;
        
        Ok(CallToolResult::success(vec![Content::text(todo_json)]))
    }

    /// Update a todo item
    #[tool(description = "Update a todo item")]
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
                    .map_err(|e| McpError::internal_error("Serialization failed", Some(json!({"error": e.to_string()}))))?;
                
                Ok(CallToolResult::success(vec![Content::text(todo_json)]))
            },
            None => Err(McpError::invalid_params(
                "Todo item with specified ID not found",
                Some(json!({"id": req.id})),
            )),
        }
    }

    /// Delete a todo item
    #[tool(description = "Delete a todo item")]
    async fn delete_todo(
        &self,
        #[tool(param)]
        #[schemars(description = "Todo item ID")]
        id: String,
    ) -> Result<CallToolResult, McpError> {
        let mut todos = self.todos.lock().await;
        let index = todos.iter().position(|t| t.id == id);
        
        match index {
            Some(idx) => {
                todos.remove(idx);
                Ok(CallToolResult::success(vec![Content::text(
                    format!("Successfully deleted todo item with ID {}", id)
                )]))
            },
            None => Err(McpError::invalid_params(
                "Todo item with specified ID not found",
                Some(json!({"id": id})),
            )),
        }
    }

    /// Get details of a single todo item
    #[tool(description = "Get details of a single todo item")]
    async fn get_todo(
        &self,
        #[tool(param)]
        #[schemars(description = "Todo item ID")]
        id: String,
    ) -> Result<CallToolResult, McpError> {
        let todos = self.todos.lock().await;
        let todo = todos.iter().find(|t| t.id == id);
        
        match todo {
            Some(todo) => {
                let todo_json = serde_json::to_string_pretty(todo)
                    .map_err(|e| McpError::internal_error("Serialization failed", Some(json!({"error": e.to_string()}))))?;
                
                Ok(CallToolResult::success(vec![Content::text(todo_json)]))
            },
            None => Err(McpError::invalid_params(
                "Todo item with specified ID not found",
                Some(json!({"id": id})),
            )),
        }
    }

    /// Mark a todo item as completed
    #[tool(description = "Mark a todo item as completed")]
    async fn complete_todo(
        &self,
        #[tool(param)]
        #[schemars(description = "Todo item ID")]
        id: String,
    ) -> Result<CallToolResult, McpError> {
        let mut todos = self.todos.lock().await;
        let todo = todos.iter_mut().find(|t| t.id == id);
        
        match todo {
            Some(todo) => {
                todo.completed = true;
                todo.updated_at = Utc::now();

                let todo_json = serde_json::to_string_pretty(&todo)
                    .map_err(|e| McpError::internal_error("Serialization failed", Some(json!({"error": e.to_string()}))))?;
                
                Ok(CallToolResult::success(vec![Content::text(todo_json)]))
            },
            None => Err(McpError::invalid_params(
                "Todo item with specified ID not found",
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
            instructions: Some("This is a todo server that helps you manage your todo list. Use list_todos to view all todos, create_todo to create new todos, update_todo to update existing todos, delete_todo to remove todos, get_todo to view todo details, and complete_todo to mark todos as completed.".to_string()),
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
