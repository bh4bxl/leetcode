// Created by bh4bxl at 2025/12/15 20:43
// leetgo: 1.4.15
// https://leetcode.com/problems/odd-even-linked-list/

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
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut odd = head;
        let mut even_head;

        {
            let odd_ref = odd.as_mut().unwrap();
            even_head = odd_ref.next.take();
        }

        let mut even = even_head.as_mut();

        let mut odd_ref = odd.as_mut();

        while even.is_some() && even.as_ref().unwrap().next.is_some() {
            // odd.next = even.next
            let next_odd = even.as_mut().unwrap().next.take();
            odd_ref.as_mut().unwrap().next = next_odd;

            // move odd
            odd_ref = odd_ref.unwrap().next.as_mut();

            // even.next = odd.next
            let next_even = odd_ref.as_mut().unwrap().next.take();
            even.as_mut().unwrap().next = next_even;

            // move even
            even = even.unwrap().next.as_mut();
        }

        // connect odd tail to even head
        odd_ref.unwrap().next = even_head;

        odd
    }
}

// @lc code=end

fn main() -> Result<()> {
    let head: LinkedList = deserialize(&read_line()?)?;
    let ans: LinkedList = Solution::odd_even_list(head.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
