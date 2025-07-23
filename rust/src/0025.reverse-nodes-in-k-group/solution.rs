// Created by bh4bxl at 2025/07/20 22:22
// leetgo: 1.4.15
// https://leetcode.com/problems/reverse-nodes-in-k-group/

use anyhow::Result;
use leetgo_rs::*;
use serde::de::IntoDeserializer;

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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut res = Some(Box::new(ListNode{ val: 0, next: None }));
        let mut cur1 = &mut head.clone();
        let mut cur2 = &mut res;

        if head == None {
            return None;
        }

        while cur1.is_some() {
            let mut stack: Vec<i32> = vec![];
            let mut i = 0;

            while i < k {
                stack.push(cur1.as_ref()?.val);
                cur1 = &mut cur1.as_mut()?.next;
                i += 1;
                if cur1.is_none() {
                    break;
                }
            }

            if i != k {
                stack.reverse();
            }

            while !stack.is_empty() {
                let v = stack.pop().unwrap();
                cur2.as_mut()?.next = Some(
                    Box::new(ListNode{ val: v, next: None})
                );
                cur2 = &mut cur2.as_mut()?.next;
            }
        }

        res.unwrap().next
    }
}

// @lc code=end

fn main() -> Result<()> {
	let head: LinkedList = deserialize(&read_line()?)?;
	let k: i32 = deserialize(&read_line()?)?;
	let ans: LinkedList = Solution::reverse_kgroup(head.into(), k).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
