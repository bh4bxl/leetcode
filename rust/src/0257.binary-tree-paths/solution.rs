// Created by bh4bxl at 2025/11/17 13:04
// leetgo: 1.4.15
// https://leetcode.com/problems/binary-tree-paths/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, path: String, res: &mut Vec<String>) {
            if let Some(n) = node {
                let np = n.borrow();
                let mut path = path.clone();
                path.push_str(&np.val.to_string());
                if np.left.is_none() && np.right.is_none() {
                    res.push(path.clone());
                }
                path.push_str("->");
                inorder(&np.left, path.clone(), res);
                inorder(&np.right, path.clone(), res);
            }
        }

        let mut res = vec![];
        inorder(&root, "".to_string(), &mut res);

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: Vec<String> = Solution::binary_tree_paths(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
