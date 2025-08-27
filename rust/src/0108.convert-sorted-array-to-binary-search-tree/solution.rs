// Created by bh4bxl at 2025/08/26 15:25
// leetgo: 1.4.15
// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/

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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree(&nums, 0, nums.len() as i32 - 1)
    }

    fn build_tree(nums: &Vec<i32>, start: i32, end: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if start > end {
            return None;
        }
        let mid = (start + end) / 2;
        let val = nums[mid as usize];
        if start == end {
            return Some(Rc::new(RefCell::new(TreeNode {
                val: val,
                left: None,
                right: None,
            })));
        }
        Some(Rc::new(RefCell::new(TreeNode {
            val: val,
            left: Self::build_tree(nums, start, mid - 1),
            right: Self::build_tree(nums, mid + 1, end),
        })))
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: BinaryTree = Solution::sorted_array_to_bst(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
