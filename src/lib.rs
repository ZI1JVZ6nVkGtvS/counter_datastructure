use std::collections::HashMap;
use std::hash::Hash;

/// 操作统计计数器
#[derive(Debug, Clone, Default)]
pub struct OperationStats {
    pub insert_count: usize,
    pub get_count: usize,
    pub delete_count: usize,
}

/// 带操作统计的 Vec 包装器
#[derive(Debug, Clone)]
pub struct CountedVec<T> {
    inner: Vec<T>,
    stats: OperationStats,
}

impl<T> CountedVec<T> {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            stats: OperationStats::default(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            stats: OperationStats::default(),
        }
    }

    /// 插入元素到末尾 (push)
    pub fn insert(&mut self, value: T) {
        self.stats.insert_count += 1;
        self.inner.push(value);
    }

    /// 获取元素 (通过索引)
    pub fn get(&mut self, index: usize) -> Option<&T> {
        self.stats.get_count += 1;
        self.inner.get(index)
    }

    /// 删除并返回指定索引的元素
    pub fn delete(&mut self, index: usize) -> Option<T> {
        self.stats.delete_count += 1;
        if index < self.inner.len() {
            Some(self.inner.remove(index))
        } else {
            None
        }
    }

    /// 获取操作统计
    pub fn stats(&self) -> &OperationStats {
        &self.stats
    }

    /// 重置操作统计
    pub fn reset_stats(&mut self) {
        self.stats = OperationStats::default();
    }

    /// 获取底层 Vec 的长度
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T> Default for CountedVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 带操作统计的 HashMap 包装器
#[derive(Debug, Clone)]
pub struct CountedHashMap<K, V> {
    inner: HashMap<K, V>,
    stats: OperationStats,
}

impl<K, V> CountedHashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            stats: OperationStats::default(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashMap::with_capacity(capacity),
            stats: OperationStats::default(),
        }
    }

    /// 插入键值对
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.stats.insert_count += 1;
        self.inner.insert(key, value)
    }

    /// 获取值 (通过键)
    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.stats.get_count += 1;
        self.inner.get(key)
    }

    /// 删除键值对
    pub fn delete(&mut self, key: &K) -> Option<V> {
        self.stats.delete_count += 1;
        self.inner.remove(key)
    }

    /// 获取操作统计
    pub fn stats(&self) -> &OperationStats {
        &self.stats
    }

    /// 重置操作统计
    pub fn reset_stats(&mut self) {
        self.stats = OperationStats::default();
    }

    /// 获取底层 HashMap 的大小
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<K, V> Default for CountedHashMap<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counted_vec_insert() {
        let mut vec = CountedVec::new();
        vec.insert(1);
        vec.insert(2);
        vec.insert(3);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec.stats().insert_count, 3);
        assert_eq!(vec.stats().get_count, 0);
        assert_eq!(vec.stats().delete_count, 0);
    }

    #[test]
    fn test_counted_vec_get() {
        let mut vec = CountedVec::new();
        vec.insert(10);
        vec.insert(20);

        assert_eq!(vec.get(0), Some(&10));
        assert_eq!(vec.get(1), Some(&20));
        assert_eq!(vec.get(2), None);

        assert_eq!(vec.stats().insert_count, 2);
        assert_eq!(vec.stats().get_count, 3);
        assert_eq!(vec.stats().delete_count, 0);
    }

    #[test]
    fn test_counted_vec_delete() {
        let mut vec = CountedVec::new();
        vec.insert(1);
        vec.insert(2);
        vec.insert(3);

        assert_eq!(vec.delete(1), Some(2));
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.delete(10), None);

        assert_eq!(vec.stats().insert_count, 3);
        assert_eq!(vec.stats().get_count, 0);
        assert_eq!(vec.stats().delete_count, 2);
    }

    #[test]
    fn test_counted_vec_reset_stats() {
        let mut vec = CountedVec::new();
        vec.insert(1);
        vec.get(0);
        vec.delete(0);

        assert_eq!(vec.stats().insert_count, 1);
        assert_eq!(vec.stats().get_count, 1);
        assert_eq!(vec.stats().delete_count, 1);

        vec.reset_stats();

        assert_eq!(vec.stats().insert_count, 0);
        assert_eq!(vec.stats().get_count, 0);
        assert_eq!(vec.stats().delete_count, 0);
    }

    #[test]
    fn test_counted_hashmap_insert() {
        let mut map = CountedHashMap::new();
        map.insert("key1", 100);
        map.insert("key2", 200);
        map.insert("key3", 300);

        assert_eq!(map.len(), 3);
        assert_eq!(map.stats().insert_count, 3);
        assert_eq!(map.stats().get_count, 0);
        assert_eq!(map.stats().delete_count, 0);
    }

    #[test]
    fn test_counted_hashmap_get() {
        let mut map = CountedHashMap::new();
        map.insert("foo", 42);
        map.insert("bar", 84);

        assert_eq!(map.get(&"foo"), Some(&42));
        assert_eq!(map.get(&"bar"), Some(&84));
        assert_eq!(map.get(&"baz"), None);

        assert_eq!(map.stats().insert_count, 2);
        assert_eq!(map.stats().get_count, 3);
        assert_eq!(map.stats().delete_count, 0);
    }

    #[test]
    fn test_counted_hashmap_delete() {
        let mut map = CountedHashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        assert_eq!(map.delete(&"a"), Some(1));
        assert_eq!(map.len(), 1);
        assert_eq!(map.delete(&"c"), None);

        assert_eq!(map.stats().insert_count, 2);
        assert_eq!(map.stats().get_count, 0);
        assert_eq!(map.stats().delete_count, 2);
    }

    #[test]
    fn test_counted_hashmap_reset_stats() {
        let mut map = CountedHashMap::new();
        map.insert("x", 1);
        map.get(&"x");
        map.delete(&"x");

        assert_eq!(map.stats().insert_count, 1);
        assert_eq!(map.stats().get_count, 1);
        assert_eq!(map.stats().delete_count, 1);

        map.reset_stats();

        assert_eq!(map.stats().insert_count, 0);
        assert_eq!(map.stats().get_count, 0);
        assert_eq!(map.stats().delete_count, 0);
    }

    #[test]
    fn test_counted_hashmap_insert_overwrites() {
        let mut map = CountedHashMap::new();
        assert_eq!(map.insert("key", 1), None);
        assert_eq!(map.insert("key", 2), Some(1));

        assert_eq!(map.stats().insert_count, 2);
        assert_eq!(map.len(), 1);
    }
}
