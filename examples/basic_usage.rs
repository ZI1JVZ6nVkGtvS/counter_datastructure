use counter_datastructure::{CountedVec, CountedHashMap};

fn main() {
    println!("=== CountedVec 示例 ===\n");
    
    // 创建一个带计数器的 Vec
    let mut vec = CountedVec::new();
    
    // 插入元素
    vec.insert(10);
    vec.insert(20);
    vec.insert(30);
    println!("插入 3 个元素后:");
    println!("  长度: {}", vec.len());
    println!("  统计: {:?}", vec.stats());
    
    // 获取元素
    let _ = vec.get(0);
    let _ = vec.get(1);
    println!("\n获取 2 个元素后:");
    println!("  统计: {:?}", vec.stats());
    
    // 删除元素
    vec.delete(1);
    println!("\n删除 1 个元素后:");
    println!("  长度: {}", vec.len());
    println!("  统计: {:?}", vec.stats());
    
    println!("\n=== CountedHashMap 示例 ===\n");
    
    // 创建一个带计数器的 HashMap
    let mut map = CountedHashMap::new();
    
    // 插入键值对
    map.insert("apple", 1);
    map.insert("banana", 2);
    map.insert("cherry", 3);
    println!("插入 3 个键值对后:");
    println!("  大小: {}", map.len());
    println!("  统计: {:?}", map.stats());
    
    // 获取值
    let _ = map.get(&"apple");
    let _ = map.get(&"banana");
    let _ = map.get(&"unknown");
    println!("\n查询 3 次后:");
    println!("  统计: {:?}", map.stats());
    
    // 删除键值对
    map.delete(&"banana");
    println!("\n删除 1 个键值对后:");
    println!("  大小: {}", map.len());
    println!("  统计: {:?}", map.stats());
    
    // 重置统计
    map.reset_stats();
    println!("\n重置统计后:");
    println!("  统计: {:?}", map.stats());
}
