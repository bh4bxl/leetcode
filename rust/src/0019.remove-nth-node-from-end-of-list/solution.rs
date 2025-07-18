// Created by bh4bxl at 2025/07/17 14:16
// leetgo: 1.4.15
// https://leetcode.com/problems/remove-nth-node-from-end-of-list/

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut first = head.as_ref();
        let mut head_m: Option<Box<ListNode>> = head.clone();
        let mut second = &mut head_m;

        for _ in 0..n {
            if first.iter().next() != None {
                first = first.unwrap().next.as_ref();
            } else {
                return None;
            }
        }

        loop {
            match first {
                Some(node) => {
                    first = node.next.as_ref();
                    second = &mut second.as_mut()?.next;
                },
                None => { break; },
            }
        }

        *second = second.as_mut().and_then(|n| n.next.take());

        head_m
    }
}

// @lc code=end

fn main() -> Result<()> {
	let head: LinkedList = deserialize(&read_line()?)?;
	let n: i32 = deserialize(&read_line()?)?;
	let ans: LinkedList = Solution::remove_nth_from_end(head.into(), n).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
