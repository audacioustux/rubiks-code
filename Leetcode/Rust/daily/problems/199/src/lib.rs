use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn traversal(node: Option<Rc<RefCell<TreeNode>>>, at_right: &mut Vec<i32>, level: usize) {
            if let Some(node) = node {
                if level == at_right.len() {
                    at_right.push(node.borrow().val);
                }
                traversal(node.borrow().right.clone(), at_right, level + 1);
                traversal(node.borrow().left.clone(), at_right, level + 1);
            }
        }
        let mut at_right = vec![];
        traversal(root, &mut at_right, 0);
        at_right
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let expected = [1, 3, 4];
        assert_eq!(Solution::right_side_view(root), expected);
    }

    #[test]
    fn testcase_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        let expected = [1, 3];
        assert_eq!(Solution::right_side_view(root), expected);
    }

    #[test]
    fn testcase_3() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: None,
        })));
        let expected = [1, 3];
        assert_eq!(Solution::right_side_view(root), expected);
    }
}
