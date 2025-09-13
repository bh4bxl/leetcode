// Created by bh4bxl at 2025/09/12 12:54
// leetgo: 1.4.15
// https://leetcode.com/problems/sum-root-to-leaf-numbers/

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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;

        fn node_num(node: &Option<Rc<RefCell<TreeNode>>>, prefix: i32, sum: &mut i32) {
            if let Some(n) = node {
                let np = n.borrow();
                let curr_pre = prefix * 10 + np.val;
                if np.left.is_none() && np.right.is_none() {
                    *sum += curr_pre;
                    return;
                }
                if np.left.is_some() {
                    node_num(&np.left, curr_pre, sum);
                }
                if np.right.is_some() {
                    node_num(&np.right, curr_pre, sum);
                }
            }
        }

        node_num(&root, 0, &mut sum);

        sum
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: i32 = Solution::sum_numbers(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
