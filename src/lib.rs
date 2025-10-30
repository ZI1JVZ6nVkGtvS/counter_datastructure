use std::collections::HashMap;

/// 操作统计信息
#[derive(Debug, Clone, Default)]
pub struct OperationStats {
    /// insert操作次数
    pub insert_count: usize,
    /// get操作次数
    pub get_count: usize,
    /// delete操作次数
    pub delete_count: usize,
}

/// 带操作统计的数据结构包装器
pub struct Counter<T> {
    inner: T,
    stats: OperationStats,
}

impl<T> Counter<T> {
    /// 创建新的Counter包装器
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            stats: OperationStats::default(),
        }
    }

    /// 获取操作统计信息
    pub fn stats(&self) -> &OperationStats {
        &self.stats
    }

    /// 获取内部数据结构的引用
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// 获取内部数据结构的可变引用
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// 解包，返回内部数据结构和统计信息
    pub fn into_inner(self) -> (T, OperationStats) {
        (self.inner, self.stats)
    }
}

impl<T> Counter<Vec<T>> {
    /// 向Vec中插入元素
    pub fn push(&mut self, value: T) {
        self.stats.insert_count += 1;
        self.inner.push(value);
    }

    /// 从Vec中获取元素
    pub fn get(&mut self, index: usize) -> Option<&T> {
        self.stats.get_count += 1;
        self.inner.get(index)
    }

    /// 从Vec中删除元素
    pub fn remove(&mut self, index: usize) -> T {
        self.stats.delete_count += 1;
        self.inner.remove(index)
    }

    /// 获取Vec长度
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// 检查Vec是否为空
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<K, V> Counter<HashMap<K, V>>
where
    K: Eq + std::hash::Hash,
{
    /// 向HashMap中插入键值对
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.stats.insert_count += 1;
        self.inner.insert(key, value)
    }

    /// 从HashMap中获取值
    pub fn get<Q>(&mut self, key: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        self.stats.get_count += 1;
        self.inner.get(key)
    }

    /// 从HashMap中删除键值对
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        self.stats.delete_count += 1;
        self.inner.remove(key)
    }

    /// 获取HashMap长度
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// 检查HashMap是否为空
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_operations() {
        let mut counter = Counter::new(Vec::new());

        // 测试插入操作
        counter.push(1);
        counter.push(2);
        counter.push(3);
        assert_eq!(counter.stats().insert_count, 3);
        assert_eq!(counter.len(), 3);

        // 测试获取操作
        assert_eq!(counter.get(0), Some(&1));
        assert_eq!(counter.get(1), Some(&2));
        assert_eq!(counter.stats().get_count, 2);

        // 测试删除操作
        let removed = counter.remove(1);
        assert_eq!(removed, 2);
        assert_eq!(counter.stats().delete_count, 1);
        assert_eq!(counter.len(), 2);

        // 验证最终统计
        let stats = counter.stats();
        assert_eq!(stats.insert_count, 3);
        assert_eq!(stats.get_count, 2);
        assert_eq!(stats.delete_count, 1);
    }

    #[test]
    fn test_hashmap_operations() {
        let mut counter = Counter::new(HashMap::new());

        // 测试插入操作
        counter.insert("a", 1);
        counter.insert("b", 2);
        counter.insert("c", 3);
        assert_eq!(counter.stats().insert_count, 3);
        assert_eq!(counter.len(), 3);

        // 测试获取操作
        assert_eq!(counter.get("a"), Some(&1));
        assert_eq!(counter.get("b"), Some(&2));
        assert_eq!(counter.stats().get_count, 2);

        // 测试删除操作
        let removed = counter.remove("b");
        assert_eq!(removed, Some(2));
        assert_eq!(counter.stats().delete_count, 1);
        assert_eq!(counter.len(), 2);

        // 验证最终统计
        let stats = counter.stats();
        assert_eq!(stats.insert_count, 3);
        assert_eq!(stats.get_count, 2);
        assert_eq!(stats.delete_count, 1);
    }

    #[test]
    fn test_into_inner() {
        let mut counter = Counter::new(Vec::new());
        counter.push(1);
        counter.push(2);

        let (vec, stats) = counter.into_inner();
        assert_eq!(vec, vec![1, 2]);
        assert_eq!(stats.insert_count, 2);
    }
}
