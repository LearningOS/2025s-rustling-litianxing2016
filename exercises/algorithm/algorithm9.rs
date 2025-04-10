/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.count += 1;
        self.items.push(value);
        self.heapify_up(self.count);
    }

    /// 上滤操作（从新添加元素开始向上调整）
    fn heapify_up(&mut self, mut idx: usize) {
        while idx > 1 {
            let parent = self.parent_idx(idx);
            match self.compare(idx, parent) {
                true => {
                    self.swap(idx, parent);
                    idx = parent;
                }
                false => break,
            }
        }
    }

    /// 下滤操作（从指定位置开始向下调整）
    fn heapify_down(&mut self, mut idx: usize) {
        while self.has_children(idx) {
            let child = self.select_child(idx);
            match self.compare(child, idx) {
                true => {  
                    self.swap(child, idx);
                    idx = child;
                }
                false => break,
            }
        }
    }

    /// 通用元素交换操作
    #[inline]
    fn swap(&mut self, i: usize, j: usize) {
        self.items.swap(i, j);
    }

    /// 通用比较操作
    #[inline]
    fn compare(&self, i: usize, j: usize) -> bool {
        let comparator = self.comparator;
        comparator(&self.items[i], &self.items[j])
    }

    /// 子节点选择逻辑（原smallest_child_idx）
    fn select_child(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        
        // 右子节点不存在时直接返回左子节点
        if right > self.count {
            return left;
        }
        
        // 根据堆类型选择合适子节点
        if self.compare(left, right) {
            left
        } else {
            right
        }
    }

    /// 更语义化的子节点存在判断
    #[inline]
    fn has_children(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		if self.count == 0 {
            return None;
        }
        
        let result = self.items.swap_remove(1);
        self.count -= 1;
        
        if self.count > 0 {
            self.heapify_down(1);
        }
        
        Some(result)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}