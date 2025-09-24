// Created by bh4bxl at 2025/09/22 12:54
// leetgo: 1.4.15
// https://leetcode.com/problems/binary-tree-preorder-traversal/

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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut list = vec![];

        fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) {
            if let Some(n) = node {
                let np = n.borrow();
                list.push(np.val);
                inorder(&np.left, list);
                inorder(&np.right, list);
            }
        }

        inorder(&root, &mut list);

        list
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::preorder_traversal(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
