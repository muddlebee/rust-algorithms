// Heap data structure
// Takes a closure as a comparator to allow for min-heap, max-heap, and works with custom key functions

use std::cmp::Ord;
use std::default::Default;

//TODO:
// - Add a remove method
// - Add a peek method
// - Add a heapify method
// - Add a heapsort method

#[derive(Clone)]
pub struct Heap<T>
    where
        T: Default + Clone,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
    where
        T: Default + Clone,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            // Add a default in the first spot to offset indexes
            // for the parent/child math to work out.
            // Vecs have to have all the same type so using Default
            // is a way to add an unused item.
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
        self.count += 1;
        self.items.push(value);

        // Heapify Up
        let mut idx = self.count;
        while self.parent_idx(idx) > 0 {
            let pdx = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[pdx]) {
                self.items.swap(idx, pdx);
            }
            idx = pdx;
        }
    }

    // delete element at index
    pub fn delete_min_heap(&mut self, idx: usize) -> Option<T> {
        if idx > self.count {
            return None;
        }
        let mut temp = self.items[idx].clone();
        self.items.swap(idx, self.count);
        self.items.pop();
        self.count -= 1;
        if idx == 1 {
            self.heapify_down(idx);
        }
        Some(temp)
    }

    //delete element at index for max heap
    pub fn delete_max_heap(&mut self, idx: usize) -> Option<T> {
        if idx > self.count {
            return None;
        }
        let mut temp = self.items[idx].clone();
        self.items.swap(idx, self.count);
        self.items.pop();
        self.count -= 1;
        if idx == 1 {
            self.heapify_down(idx);
        }
        Some(temp)
    }


    // write a function to heapify down
    pub fn heapify_down(&mut self, mut idx: usize) {
        if idx > self.count {
            return;
        }
        while self.children_present(idx) {
            let smallest_child_idx = self.smallest_child_idx(idx);
            if (self.comparator)(&self.items[smallest_child_idx], &self.items[idx]) {
                self.items.swap(idx, smallest_child_idx);
            }
            idx = smallest_child_idx;
        }
    }

    // write a function to heapify up
    pub fn heapify_up(&mut self, mut idx: usize) {
        if idx > self.count {
            return;
        }
        while self.parent_present(idx) {
            let parent_idx = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
            }
            idx = parent_idx;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn parent_present(&self, idx: usize) -> bool {
        self.parent_idx(idx) > 0
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

    // Returns the index of the smallest child
    fn smallest_child_idx(&self, idx: usize) -> usize {
        if self.right_child_idx(idx) > self.count {
            self.left_child_idx(idx)
        } else {
            let ldx = self.left_child_idx(idx);
            let rdx = self.right_child_idx(idx);
            if (self.comparator)(&self.items[ldx], &self.items[rdx]) {
                ldx
            } else {
                rdx
            }
        }
    }
}

impl<T> Heap<T>
    where
        T: Default + Ord + Clone,
{
    /// Create a new MinHeap
    pub fn new_min() -> Heap<T> {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Heap<T> {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
    where
        T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        // This feels like a function built for heap impl :)
        // Removes an item at an index and fills in with the last item
        // of the Vec
        let next = Some(self.items.swap_remove(1));
        self.count -= 1;

        if self.count > 0 {
            // Heapify Down
            let mut idx = 1;
            while self.children_present(idx) {
                let cdx = self.smallest_child_idx(idx);
                if !(self.comparator)(&self.items[idx], &self.items[cdx]) {
                    self.items.swap(idx, cdx);
                }
                idx = cdx;
            }
        }

        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap: Heap<i32> = Heap::new_max();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = Heap::new_min();
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
        let mut heap = Heap::new_max();
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

    #[derive(Clone)]
    struct Point(/* x */ i32, /* y */ i32);

    impl Default for Point {
        fn default() -> Self {
            Self(0, 0)
        }
    }

    #[test]
    fn test_key_heap() {
        let mut heap: Heap<Point> = Heap::new(|a, b| a.0 < b.0);
        heap.add(Point(1, 5));
        heap.add(Point(3, 10));
        heap.add(Point(-2, 4));
        assert_eq!(heap.len(), 3);
        assert_eq!(heap.next().unwrap().0, -2);
        assert_eq!(heap.next().unwrap().0, 1);
        heap.add(Point(50, 34));
        assert_eq!(heap.next().unwrap().0, 3);
    }

    //tests for smallest_child_idx
    #[test]
    fn test_smallest_child_idx() {
        let mut heap = Heap::new_min();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        println!("size of heap {:?}", heap.len());
        assert_eq!(heap.smallest_child_idx(1), 2);
        assert_eq!(heap.smallest_child_idx(2), 4);
        assert_eq!(heap.smallest_child_idx(3), 6);
    }

    //tests for heapify_up
    #[test]
    fn test_heapify_up() {
        let mut heap = Heap::new_max();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        heap.heapify_up(1);
        println!("heap items {:?}", heap.items);
        assert_eq!(heap.items[1], 11);
    }

    //tests for heapify_down
    #[test]
    fn test_heapify_down() {
        let mut heap = Heap::new_min();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        heap.heapify_down(1);
        println!("heap items {:?}", heap.items);
        assert_eq!(heap.items[1], 2);
    }

    //tests for delete_max_heap
    #[test]
    fn test_delete_max_heap() {
        let mut heap = Heap::new_max();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        heap.delete_max_heap(1);
        println!("heap items {:?}", heap.items);
        assert_eq!(heap.items[1], 9);
    }

    //tests for delete_min_heap
    #[test]
    fn test_delete_min_heap() {
        let mut heap = Heap::new_min();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        heap.delete_min_heap(1);
        println!("heap items {:?}", heap.items);
        assert_eq!(heap.items[1], 4);
    }
}
