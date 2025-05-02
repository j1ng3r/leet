use binomial_heap::bheap::BHeap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_enqueue_and_find_min() {
        let mut heap = BHeap::new();
        heap.enqueue(42);
        assert_eq!(heap.find_min(), Some(42));
    }

    #[test]
    fn test_multiple_enqueues_and_find_min() {
        let mut heap = BHeap::new();
        heap.enqueue(10);
        heap.enqueue(3);
        heap.enqueue(8);
        heap.enqueue(1);
        heap.enqueue(15);
        assert_eq!(heap.find_min(), Some(1));
    }

    #[test]
    fn test_extract_min_sequence() {
        let mut heap = BHeap::new();
        for &x in &[12, 7, 5, 20, 2, 9] {
            heap.enqueue(x);
        }

        let mut results = Vec::new();
        while let Some(min) = heap.extract_min() {
            results.push(min);
        }

        assert_eq!(results, vec![2, 5, 7, 9, 12, 20]);
        assert_eq!(heap.find_min(), None);
    }

    #[test]
    fn test_extract_min_on_empty_heap() {
        let mut heap = BHeap::<i32>::new();
        assert_eq!(heap.extract_min(), None);
    }

    #[test]
    fn test_find_min_on_empty_heap() {
        let heap = BHeap::<i32>::new();
        assert_eq!(heap.find_min(), None);
    }

    #[test]
    fn test_meld_two_heaps() {
        let mut h1 = BHeap::new();
        h1.enqueue(5);
        h1.enqueue(20);
        h1.enqueue(3);

        let mut h2 = BHeap::new();
        h2.enqueue(7);
        h2.enqueue(2);
        h2.enqueue(15);

        h1.meld(h2);

        let mut results = Vec::new();
        while let Some(min) = h1.extract_min() {
            results.push(min);
        }

        assert_eq!(results, vec![2, 3, 5, 7, 15, 20]);
    }

    #[test]
    fn test_meld_with_empty_heap() {
        let mut h1 = BHeap::new();
        h1.enqueue(4);
        h1.enqueue(9);

        let h2 = BHeap::<i32>::new();
        h1.meld(h2);

        assert_eq!(h1.find_min(), Some(4));
    }

    #[test]
    fn test_extract_min_does_not_duplicate_values() {
        let mut heap = BHeap::new();
        heap.enqueue(3);
        heap.enqueue(1);
        heap.enqueue(4);

        assert_eq!(heap.extract_min(), Some(1));
        assert!(heap.extract_min().is_some());
        assert!(heap.extract_min().is_some());
        assert_eq!(heap.extract_min(), None);
    }

    #[test]
    fn test_heap_reuse_after_extract() {
        let mut heap = BHeap::new();
        heap.enqueue(9);
        heap.enqueue(1);
        heap.enqueue(6);

        assert_eq!(heap.extract_min(), Some(1));
        heap.enqueue(0);
        assert_eq!(heap.find_min(), Some(0));
    }
}
