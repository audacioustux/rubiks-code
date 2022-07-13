use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut level_ordered: Vec<Vec<i32>> = Vec::new();

        fn traversal(
            root: Option<Rc<RefCell<TreeNode>>>,
            level_ordered: &mut Vec<Vec<i32>>,
            level: usize,
        ) {
            if let Some(node) = root {
                if level_ordered.len() == level {
                    level_ordered.push(Vec::new());
                }
                level_ordered[level].push(node.borrow().val);
                traversal(node.borrow().left.clone(), level_ordered, level + 1);
                traversal(node.borrow().right.clone(), level_ordered, level + 1);
            }
        }
        traversal(root.clone(), &mut level_ordered, 0);

        level_ordered
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
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let expected = [vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(Solution::level_order(root), expected);
    }

    #[test]
    fn testcase_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let expected = [[1]];
        assert_eq!(Solution::level_order(root), expected);
    }

    #[test]
    fn testcase_3() {
        let root = None;
        let expected: Vec<Vec<_>> = vec![];
        assert_eq!(Solution::level_order(root), expected);
    }
}
