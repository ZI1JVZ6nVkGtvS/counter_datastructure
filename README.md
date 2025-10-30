# counter_datastructure
数据结构次数统计

一个 Rust 库，用于跟踪数据结构操作的次数（insert, get, delete）。

## 功能特性

- ✅ 支持 `Vec` 的操作统计（`CountedVec`）
- ✅ 支持 `HashMap` 的操作统计（`CountedHashMap`）
- 📊 跟踪 insert、get、delete 操作次数
- 🔄 支持重置统计计数器

## 使用示例

### CountedVec

```rust
use counter_datastructure::CountedVec;

let mut vec = CountedVec::new();

// 插入元素
vec.insert(10);
vec.insert(20);
vec.insert(30);

// 获取元素
let value = vec.get(0);

// 删除元素
vec.delete(1);

// 查看统计
println!("统计: {:?}", vec.stats());
// 输出: OperationStats { insert_count: 3, get_count: 1, delete_count: 1 }
```

### CountedHashMap

```rust
use counter_datastructure::CountedHashMap;

let mut map = CountedHashMap::new();

// 插入键值对
map.insert("key1", 100);
map.insert("key2", 200);

// 获取值
let value = map.get(&"key1");

// 删除键值对
map.delete(&"key1");

// 查看统计
println!("统计: {:?}", map.stats());
// 输出: OperationStats { insert_count: 2, get_count: 1, delete_count: 1 }
```

## 运行示例

```bash
cargo run --example basic_usage
```

## 运行测试

```bash
cargo test
```
