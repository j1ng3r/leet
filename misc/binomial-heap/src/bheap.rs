use std::cmp::Ord;

#[derive(Debug)]
pub struct BTree<T> {
    val: T,
    ptrs: Vec<Box<BTree<T>>>,
}

fn merge_btrees<T: Ord>(mut a: Box<BTree<T>>, mut b: Box<BTree<T>>) -> Box<BTree<T>> {
    if a.val <= b.val {
        a.ptrs.push(b);
        a
    } else {
        b.ptrs.push(a);
        b
    }
}

#[derive(Debug)]
pub struct BHeap<T> {
    slots: Vec<Option<Box<BTree<T>>>>,
}

impl<T: Ord + Clone> BHeap<T> {
    pub fn new() -> Self {
        BHeap {
            slots: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, val: T) {
        let mut active = Box::new(BTree { val, ptrs: Vec::new() });
        let mut i = 0;

        while i < self.slots.len() {
            // Moving ownership out of an Option<T> safely — without cloning and without leaving the slot uninitialized.
            match self.slots[i].take() {
                None => {
                    self.slots[i] = Some(active);
                    return;
                }
                Some(existing) => {
                    active = merge_btrees(active, existing);
                    // Continue looping, current slot is now empty.
                }
            }
            i += 1;
        }
        self.slots.push(Some(active));
    }

    pub fn find_min(&self) -> Option<T> {
        let mut min_val = None;

        for slot in &self.slots {
            if let Some(tree) = slot {
                match &min_val {
                    None => min_val = Some(tree.val.clone()),
                    Some(curr_min) if tree.val < *curr_min => {
                        min_val = Some(tree.val.clone())
                    }
                    _ => {}
                }
            }
        }

        min_val
    }

    pub fn extract_min(&mut self) -> Option<T> {
        let mut min_item: Option<(T, usize)> = None;

        for (i, slot) in self.slots.iter().enumerate() {
            if let Some(tree) = slot {
                match &min_item {
                    None => {
                        min_item = Some((tree.val.clone(), i));
                    }
                    Some((curr_min, _)) if tree.val < *curr_min => {
                        min_item = Some((tree.val.clone(), i));
                    }
                    _ => {}
                }
            }
        }

        match min_item {
            None => None,
            Some((min_val, mindex)) => {
                // Remove the tree with the min root
                let min_tree = self.slots[mindex].take().unwrap();
                
                // Split up the children and meld them back to the heap 
                let mut carry: Option<Box<BTree<T>>> = None;
                let mut i: usize = 0;
                for child in min_tree.ptrs.into_iter() {
                    match carry.take() {
                        None => {
                            match self.slots[i].take() {
                                None => {
                                    self.slots[i] = Some(child);
                                }
                                Some(self_b) => {
                                    carry = Some(merge_btrees(self_b, child));
                                }
                            }
                        }
                        Some(carry_b) => {
                            carry = Some(merge_btrees(carry_b, child));
                        }
                    }
                    i += 1;
                }
                match carry.take() {
                    None => {}
                    Some(mut carry_b) => {
                        while let Some(self_b) = self.slots[i].take() {
                            carry_b = merge_btrees(self_b, carry_b);
                            i += 1;
                        }
                        self.slots[i] = Some(carry_b);
                    }
                }
                Some(min_val)
            }
        }
    }

    pub fn meld(&mut self, other: BHeap<T>) {
        let mut carry: Option<Box<BTree<T>>> = None;

        let mut i: usize = 0;
        for other_b in other.slots.into_iter() {
            if self.slots.len() <= i {
                self.slots.push(None);
            }
            match other_b {
                None => match carry.take() {
                    None => {}
                    Some(carry_b) => match self.slots[i].take() {
                        None => {
                            self.slots[i] = Some(carry_b)
                        }
                        Some(self_b) => {
                            carry = Some(merge_btrees(carry_b, self_b));
                        }
                    }
                }
                Some(slot_b) => match carry.take() {
                    None => match self.slots[i].take() {
                        None => {
                            self.slots[i] = Some(slot_b)
                        }
                        Some(self_b) => {
                            carry = Some(merge_btrees(self_b, slot_b));
                        }
                    }
                    Some(carry_b) => {
                        carry = Some(merge_btrees(carry_b, slot_b));
                    }
                }
            }
            i += 1;
        }
        match carry.take() {
            None => {}
            Some(mut carry_b) => {
                while self.slots.len() <= i {
                    self.slots.push(None);
                }
                while let Some(self_b) = self.slots[i].take() {
                    carry_b = merge_btrees(self_b, carry_b);
                    i += 1;
                    if self.slots.len() <= i {
                        self.slots.push(None);
                    }
                }
                self.slots[i] = Some(carry_b);
            }
        }
    }
}