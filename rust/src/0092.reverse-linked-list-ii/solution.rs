// Created by bh4bxl at 2025/08/19 15:15
// leetgo: 1.4.15
// https://leetcode.com/problems/reverse-linked-list-ii/

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
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut prev = &mut dummy;

        for _ in 0..left - 1 {
            prev = prev.next.as_mut().unwrap();
        }

        let mut curr = prev.next.take();

        for _ in 0..right - left {
            let mut next = curr.as_mut()?.next.take();
            curr.as_mut()?.next = next.as_mut()?.next.take();
            next.as_mut()?.next = prev.next.take();
            prev.next = next;
        }

        if prev.next.is_none() {
            prev.next = curr;
        } else {
            let mut last = prev.next.as_mut()?;
            while last.next.is_some() {
                last = last.next.as_mut()?;
            }
            last.next = curr;
        }

        dummy.next
    }
}

// @lc code=end

fn main() -> Result<()> {
    let head: LinkedList = deserialize(&read_line()?)?;
    let left: i32 = deserialize(&read_line()?)?;
    let right: i32 = deserialize(&read_line()?)?;
    let ans: LinkedList = Solution::reverse_between(head.into(), left, right).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
