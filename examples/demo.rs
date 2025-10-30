use counter_datastructure::Counter;
use std::collections::HashMap;

fn main() {
    println!("=== Vec 示例 ===");
    demo_vec();
    
    println!("\n=== HashMap 示例 ===");
    demo_hashmap();
}

fn demo_vec() {
    let mut counter = Counter::new(Vec::new());
    
    // 插入元素
    counter.push(10);
    counter.push(20);
    counter.push(30);
    counter.push(40);
    println!("插入了 4 个元素");
    
    // 获取元素
    println!("获取索引 0 的元素: {:?}", counter.get(0));
    println!("获取索引 2 的元素: {:?}", counter.get(2));
    
    // 删除元素
    let removed = counter.remove(1);
    println!("删除了元素: {}", removed);
    
    // 显示统计信息
    let stats = counter.stats();
    println!("\n操作统计:");
    println!("  插入次数: {}", stats.insert_count);
    println!("  获取次数: {}", stats.get_count);
    println!("  删除次数: {}", stats.delete_count);
    
    // 解包并显示最终的Vec内容
    let (vec, _) = counter.into_inner();
    println!("\n最终的Vec内容: {:?}", vec);
}

fn demo_hashmap() {
    let mut counter = Counter::new(HashMap::new());
    
    // 插入键值对
    counter.insert("user_id", 1001);
    counter.insert("age", 25);
    counter.insert("score", 95);
    println!("插入了 3 个键值对");
    
    // 获取值
    println!("获取 'user_id': {:?}", counter.get("user_id"));
    println!("获取 'score': {:?}", counter.get("score"));
    println!("获取 'unknown': {:?}", counter.get("unknown"));
    
    // 删除键值对
    let removed = counter.remove("age");
    println!("删除了键值对 'age': {:?}", removed);
    
    // 显示统计信息
    let stats = counter.stats();
    println!("\n操作统计:");
    println!("  插入次数: {}", stats.insert_count);
    println!("  获取次数: {}", stats.get_count);
    println!("  删除次数: {}", stats.delete_count);
    
    // 解包并显示最终的HashMap内容
    let (map, _) = counter.into_inner();
    println!("\n最终的HashMap内容: {:?}", map);
}
