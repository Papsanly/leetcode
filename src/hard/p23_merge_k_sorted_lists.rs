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

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // Min-heap of (head value, list index), one entry per non-empty list.
        let mut heap: BinaryHeap<_> = lists
            .iter()
            .enumerate()
            .filter_map(|(i, list)| Some((Reverse(list.as_ref()?.val), i)))
            .collect();

        let mut head = None;
        let mut tail = &mut head;
        while let Some((_, i)) = heap.pop() {
            let mut smallest = lists[i]
                .take()
                .expect("heap only holds indices of non-empty lists");
            lists[i] = smallest.next.take();
            if let Some(new_head) = &lists[i] {
                heap.push((Reverse(new_head.val), i));
            }
            tail = &mut tail.insert(smallest).next;
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

    #[test]
    fn empty_lists_mixed_with_non_empty() {
        assert_eq!(
            Solution::merge_k_lists(vec![list![], list![1, 3], list![], list![2], list![]]),
            list![1, 2, 3]
        )
    }

    #[test]
    fn negative_values_and_duplicates_across_lists() {
        assert_eq!(
            Solution::merge_k_lists(vec![list![-10, -3, 0], list![-5, -3, 7], list![-3]]),
            list![-10, -5, -3, -3, -3, 0, 7]
        )
    }
}
