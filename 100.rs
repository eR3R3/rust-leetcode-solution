// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(&root.left, &root.right)
    }

    pub fn dfs(n1: &Node, n2: &Node) {
        match (n1, n2) {
            (Some(node1), Some(node2)) => {
                node1.val == node2.val &&
                Self::dfs(&node1.left, &node2.right) && 
                Self::dfs(&node1.right, &node2.left)
            },
            (None, None) => true,
            (_, _) => false,
        }
    }
}
