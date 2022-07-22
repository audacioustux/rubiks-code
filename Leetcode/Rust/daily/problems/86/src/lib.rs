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

impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut left: Vec<i32> = vec![];
        let mut right: Vec<i32> = vec![];
        while let Some(node) = head {
            let val = node.val;
            if val < x {
                left.push(val);
            } else {
                right.push(val);
            }
            head = node.next;
        }
        let mut next = None;
        while let Some(val) = right.pop() {
            next = Some(Box::new(ListNode { next, val }));
        }
        while let Some(val) = left.pop() {
            next = Some(Box::new(ListNode { next, val }));
        }
        next
    }
}

#[cfg(test)]
mod tests {}
