pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut data = vec![];
        while let Some(mut node) = head {
            head = node.next;
            node.next = None;
            data.push(node);
        }

        let (i, j) = (left as usize - 1, right as usize - 1);
        data[i..=j].reverse();

        let mut ret = None;
        let mut tail = &mut ret;
        for p in data {
            tail = &mut tail.insert(p).next;
        }

        ret
    }
}
