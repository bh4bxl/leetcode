// Created by bh4bxl at 2025/11/04 13:11
// leetgo: 1.4.15
// https://leetcode.com/problems/kth-smallest-element-in-a-bst/

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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut result = vec![];

        fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>, k: i32) {
            if let Some(n) = node {
                let np = n.borrow();
                inorder(&np.left, result, k);
                result.push(np.val);
                if result.len() == k as usize {
                    return;
                }
                inorder(&np.right, result, k);
            }
        }

        inorder(&root, &mut result, k);

        result[k as usize - 1]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::kth_smallest(root.into(), k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
