// Created by bh4bxl at 2025/08/26 21:06
// leetgo: 1.4.15
// https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut list = vec![];
        let mut curr = head.clone();

        while let Some(node) = curr {
            let val = node.val;
            list.push(val);
            curr = node.next;
        }

        Self::build_tree(&list, 0, list.len() as i32 - 1)
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
    let head: LinkedList = deserialize(&read_line()?)?;
    let ans: BinaryTree = Solution::sorted_list_to_bst(head.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
