# MCP Todo List 服务器

这是一个使用Rust编写的Model Context Protocol (MCP)待办事项服务器，提供了基本的待办事项管理功能。

## 功能

- 列出所有待办事项
- 创建新的待办事项
- 更新现有待办事项
- 删除待办事项
- 获取待办事项详情
- 标记待办事项为已完成

## 构建和运行

### 构建

```bash
cargo build --release
```

### 运行

```bash
cargo run --release
```

## 在Claude Desktop中使用

1. 构建服务器

```bash
cargo build --release
```

2. 在Claude Desktop的配置文件中添加服务器配置

```json
{
  "mcpServers": {
    "todolist": {
      "command": "PATH-TO/mcp-todo-server/target/release/mcp-todo-server",
      "args": []
    }
  }
}
```

3. 重启Claude Desktop

4. 使用以下工具与待办事项服务器交互：

- `todolist.list_todos` - 列出所有待办事项
- `todolist.create_todo` - 创建新的待办事项，参数: `{"title": "待办事项标题", "description": "可选描述"}`
- `todolist.update_todo` - 更新待办事项，参数: `{"id": "待办事项ID", "title": "新标题", "description": "新描述", "completed": true|false}`
- `todolist.delete_todo` - 删除待办事项，参数: `"待办事项ID"`
- `todolist.get_todo` - 获取待办事项详情，参数: `"待办事项ID"`
- `todolist.complete_todo` - 标记待办事项为已完成，参数: `"待办事项ID"`

## 示例

```
// 创建一个新的待办事项
todolist.create_todo {"title": "购买牛奶", "description": "超市购买1升鲜牛奶"}

// 列出所有待办事项
todolist.list_todos

// 完成一个待办事项
todolist.complete_todo "待办事项ID"

// 获取待办事项详情
todolist.get_todo "待办事项ID"
```

## 技术栈

- Rust
- tokio - 异步运行时
- rmcp - Model Context Protocol (MCP) SDK
- serde - 序列化和反序列化 
