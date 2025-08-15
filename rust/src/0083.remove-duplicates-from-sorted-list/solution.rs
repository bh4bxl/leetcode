// Created by bh4bxl at 2025/08/14 12:29
// leetgo: 1.4.15
// https://leetcode.com/problems/remove-duplicates-from-sorted-list/

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
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head_m = head.clone();
        let mut curr = head_m.as_mut();

        while let Some(node) = curr {
            while let Some(next_node) = node.next.as_mut() {
                if next_node.val == node.val {
                    node.next = next_node.next.take();
                } else {
                    break;
                }
            }
            curr = node.next.as_mut();
        }

        head_m
    }
}

// @lc code=end

fn main() -> Result<()> {
    let head: LinkedList = deserialize(&read_line()?)?;
    let ans: LinkedList = Solution::delete_duplicates(head.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
