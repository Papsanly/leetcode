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

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1_current = l1.as_ref();
        let mut l2_current = l2.as_ref();
        let mut head = None;
        let mut tail = &mut head;
        let mut carry = 0;
        while l1_current.is_some() || l2_current.is_some() || carry == 1 {
            let val1 = l1_current.as_ref().map(|node| node.val).unwrap_or(0);
            let val2 = l2_current.as_ref().map(|node| node.val).unwrap_or(0);
            let sum = val1 + val2 + carry;
            let res = sum % 10;
            carry = sum / 10;
            *tail = Some(Box::new(ListNode::new(res)));
            tail = &mut tail.as_mut().unwrap().next;
            l1_current = l1_current.and_then(|node| node.next.as_ref());
            l2_current = l2_current.and_then(|node| node.next.as_ref());
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
            Solution::add_two_numbers(dbg!(list![2, 4, 3]), dbg!(list![5, 6, 4])),
            list![7, 0, 8],
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::add_two_numbers(list![0], list![0]), list![0]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            Solution::add_two_numbers(list![9, 9, 9, 9, 9, 9, 9], list![9, 9, 9, 9]),
            list![8, 9, 9, 9, 0, 0, 0, 1]
        );
    }
}
