---
name: rust developer
description: rust的开发ai
---

# rust

## 注释规范

**核心原则**

- 简洁性优先：注释应当简洁明了，避免冗余
- 业务语义：注释应描述业务逻辑而非代码实现
  - ❌ 错误示例：`// 检测x是否为1`
  - ✅ 正确示例：`// 检测用户是否登录`
- 优先通过清晰的变量名和代码结构来表达意图

**具体规则**

1. impl块顶部不添加注释
2. `impl Trait for T` 中不添加注释，除非非常复杂

3. `pub`的函数打///文档注释，`pub`的mod在顶部打//!文档注释

## 命名规范

| 类型       | 规则                                 | 示例                                                         |
| ---------- | ------------------------------------ | ------------------------------------------------------------ |
| **Struct** | 名词                                 | `MonitorCaptor`, `WindowCaptor`, `User`, `TaskRepository`    |
| **Trait**  | **优先**：形容词（能力）             | `Debug`, `Clone`, `Serializable`, `Send`, `Sync`             |
|            | **可选**：名词（抽象）或动词（动作） | `Iterator`, `Capture`, `Handler`                             |
| **函数**   | 动词/动名词，省略 `get_`             | `create_user()`, `update_task()`, `user(id)`, `active_users()` |
| **字段**   | 名词/形容词                          | `user_name`, `is_active`, `created_at`                       |
| **目录**   | 功能名（单数）或领域名（复数）       | `capture/`, `services/`, `models/`                           |
| **文件**   | 具体实现或子模块                     | `capture/window.rs`, `models/user.rs`                        |

## 目录与文件组织原则

| 组织方式     | 适用场景     | 示例                                                         |
| ------------ | ------------ | ------------------------------------------------------------ |
| **功能组织** | 相关功能聚合 | `capture/` 目录下有 `window.rs`（WindowCaptor）、`screen.rs`（ScreenCaptor） |
| **领域组织** | 业务实体聚合 | `models/` 目录下有 `user.rs`、`task.rs`                      |
| **层级组织** | 架构层次分离 | `services/`、`repositories/`、`handlers/`                    |

> **建议**：目录组织方式很合适，既支持功能聚合又支持领域划分，灵活性好。可以根据模块大小选择：
>
> - 小模块：直接用文件（`user.rs`）
> - 大模块：用目录组织（`user/mod.rs`, `user/dto.rs`, `user/entity.rs`

## **pub原则**

* 尽量不要用pub(crate)这种写法，而是非pub的模块不要pub，如果该模块有函数在外面用pub use
* 子模块应默认私有，在父模块pub use

