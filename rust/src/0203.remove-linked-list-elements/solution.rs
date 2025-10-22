// Created by bh4bxl at 2025/10/20 14:53
// leetgo: 1.4.15
// https://leetcode.com/problems/remove-linked-list-elements/

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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut curr = &mut dummy;

        while let Some(ref mut node) = curr.next {
            if node.val == val {
                curr.next = node.next.take();
            } else {
                curr = curr.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}

// @lc code=end

fn main() -> Result<()> {
    let head: LinkedList = deserialize(&read_line()?)?;
    let val: i32 = deserialize(&read_line()?)?;
    let ans: LinkedList = Solution::remove_elements(head.into(), val).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
