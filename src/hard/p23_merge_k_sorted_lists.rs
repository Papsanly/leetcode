struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

macro_rules! list {
    ($first:expr $(, $rest:expr)*) => {
        {
            let mut node = Box::new(ListNode::new($first));
            node.next = list![$($rest),*];
            Some(node)
        }
    };
    () => {
        None
    };
}

use std::{cmp, collections::BinaryHeap, mem};

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;
        let mut heap = BinaryHeap::new();
        for node in lists.into_iter().flatten() {
            heap.push(node);
        }
        while let Some(mut node) = heap.pop() {
            let mut next = mem::take(&mut node.next);
            *tail = Some(node);
            if let Some(node) = tail {
                tail = &mut node.next;
            }
            if let Some(node) = next {
                heap.push(node);
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::merge_k_lists(vec![list![1, 4, 5], list![1, 3, 4], list![2, 6]]),
            list![1, 1, 2, 3, 4, 4, 5, 6]
        )
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::merge_k_lists(vec![]), list![])
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::merge_k_lists(vec![list![]]), list![])
    }

    #[test]
    fn test_case_4() {
        assert_eq!(
            Solution::merge_k_lists(vec![list![0, 2, 5]]),
            list![0, 2, 5]
        )
    }
}
