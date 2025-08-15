// Created by bh4bxl at 2025/08/13 21:55
// leetgo: 1.4.15
// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/

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
        let mut res_head = Box::new(ListNode { val: 0, next: head });
        let mut res_curr = &mut res_head;

        while let Some(mut curr) = res_curr.next.take() {
            let mut repeat = false;

            while let Some(next_node) = &curr.next {
                if next_node.val == curr.val {
                    repeat = true;
                    curr.next = curr.next.as_mut()?.next.take();
                } else {
                    break;
                }
            }

            if repeat {
                res_curr.next = curr.next.take();
            } else {
                res_curr.next = Some(curr);
                res_curr = res_curr.next.as_mut()?;
            }
        }

        res_head.next
    }
}

// @lc code=end

fn main() -> Result<()> {
    let head: LinkedList = deserialize(&read_line()?)?;
    let ans: LinkedList = Solution::delete_duplicates(head.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
