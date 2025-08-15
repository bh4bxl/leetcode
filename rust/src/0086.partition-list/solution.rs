// Created by bh4bxl at 2025/08/14 16:09
// leetgo: 1.4.15
// https://leetcode.com/problems/partition-list/

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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut curr = head;
        let mut less_head = Box::new(ListNode { val: 0, next: None });
        let mut less_curr = &mut less_head;
        let mut more_head = Box::new(ListNode { val: 0, next: None });
        let mut more_curr = &mut more_head;

        while let Some(mut node) = curr {
            curr = node.next.take();
            if node.val < x {
                less_curr.next = Some(node);
                less_curr = less_curr.next.as_mut()?;
            } else {
                more_curr.next = Some(node);
                more_curr = more_curr.next.as_mut()?;
            }
        }

        more_curr.next = None;
        less_curr.next = more_head.next;

        less_head.next
    }
}

// @lc code=end

fn main() -> Result<()> {
    let head: LinkedList = deserialize(&read_line()?)?;
    let x: i32 = deserialize(&read_line()?)?;
    let ans: LinkedList = Solution::partition(head.into(), x).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
