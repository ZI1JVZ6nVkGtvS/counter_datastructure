# counter_datastructure
数据结构次数统计

一个Rust库，用于跟踪数据结构操作（insert、get、delete）的次数。

## 功能特性

- ✅ 支持 `Vec` 数据结构
- ✅ 支持 `HashMap` 数据结构
- 跟踪 insert、get、delete 操作次数

## 使用示例

### Vec 使用示例

```rust
use counter_datastructure::Counter;

fn main() {
    let mut counter = Counter::new(Vec::new());
    
    // 插入元素
    counter.push(1);
    counter.push(2);
    counter.push(3);
    
    // 获取元素
    let value = counter.get(0);
    
    // 删除元素
    counter.remove(1);
    
    // 查看操作统计
    let stats = counter.stats();
    println!("插入次数: {}", stats.insert_count);
    println!("获取次数: {}", stats.get_count);
    println!("删除次数: {}", stats.delete_count);
}
```

### HashMap 使用示例

```rust
use counter_datastructure::Counter;
use std::collections::HashMap;

fn main() {
    let mut counter = Counter::new(HashMap::new());
    
    // 插入键值对
    counter.insert("key1", "value1");
    counter.insert("key2", "value2");
    
    // 获取值
    let value = counter.get("key1");
    
    // 删除键值对
    counter.remove("key2");
    
    // 查看操作统计
    let stats = counter.stats();
    println!("插入次数: {}", stats.insert_count);
    println!("获取次数: {}", stats.get_count);
    println!("删除次数: {}", stats.delete_count);
}
```

## API 文档

### `Counter<T>`

通用包装器类型，可以包装任意数据结构并跟踪操作次数。

#### 方法

- `new(inner: T) -> Self` - 创建新的计数器包装器
- `stats(&self) -> &OperationStats` - 获取操作统计信息
- `inner(&self) -> &T` - 获取内部数据结构的引用
- `inner_mut(&mut self) -> &mut T` - 获取内部数据结构的可变引用
- `into_inner(self) -> (T, OperationStats)` - 解包，返回内部数据结构和统计信息

### `Counter<Vec<T>>` 特定方法

- `push(&mut self, value: T)` - 插入元素（计入insert_count）
- `get(&mut self, index: usize) -> Option<&T>` - 获取元素（计入get_count）
- `remove(&mut self, index: usize) -> T` - 删除元素（计入delete_count）
- `len(&self) -> usize` - 获取Vec长度
- `is_empty(&self) -> bool` - 检查Vec是否为空

### `Counter<HashMap<K, V>>` 特定方法

- `insert(&mut self, key: K, value: V) -> Option<V>` - 插入键值对（计入insert_count）
- `get<Q>(&mut self, key: &Q) -> Option<&V>` - 获取值（计入get_count）
- `remove<Q>(&mut self, key: &Q) -> Option<V>` - 删除键值对（计入delete_count）
- `len(&self) -> usize` - 获取HashMap长度
- `is_empty(&self) -> bool` - 检查HashMap是否为空

### `OperationStats`

操作统计信息结构。

#### 字段

- `insert_count: usize` - insert操作次数
- `get_count: usize` - get操作次数
- `delete_count: usize` - delete操作次数

