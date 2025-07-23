// Created by bh4bxl at 2025/07/18 19:57
// leetgo: 1.4.15
// https://leetcode.com/problems/swap-nodes-in-pairs/

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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res = Some(Box::new(ListNode { val: 0, next: None }));
        let mut cur1 = &mut head.clone();
        let mut cur2 = &mut res;

        if head == None {
            return None;
        }

        while cur1.is_some() {
            if cur1.as_ref()?.next == None {
                let v = cur1.as_ref()?.val;
                cur2.as_mut()?.next = Some(
                    Box::new(ListNode { val: v, next: None })
                );
                break;
            }
            let v1 = cur1.as_ref()?.val;
            let v2 = cur1.as_ref()?.next.as_ref()?.val;

            cur2.as_mut()?.next = Some(
                Box::new(
                    ListNode { val: v2, next: Some(
                        Box::new(ListNode { val: v1, next: None })
                    ) }));

            cur1 = &mut cur1.as_mut()?.next.as_mut()?.next;
            cur2 = &mut cur2.as_mut()?.next.as_mut()?.next;
        }

        res.unwrap().next
    }
}

// @lc code=end

fn main() -> Result<()> {
	let head: LinkedList = deserialize(&read_line()?)?;
	let ans: LinkedList = Solution::swap_pairs(head.into()).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
