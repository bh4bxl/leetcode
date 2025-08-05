// Created by bh4bxl at 2025/08/01 13:53
// leetgo: 1.4.15
// https://leetcode.com/problems/rotate-list/

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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head == None {
            return None;
        }

        let mut curr = &mut head.clone();
        let mut nums = vec![];

        while curr.is_some() {
            nums.push(curr.as_ref()?.val);
            curr = &mut curr.as_mut()?.next;
        }

        let len = nums.len();
        let move_num = k % len as i32;
        if move_num == 0 {
            return head.clone();
        }

        let mut res = Some(Box::new(ListNode { next: None, val: 0 }));
        let mut curr = &mut res;

        for i in 0..nums.len() {
            curr.as_mut()?.next = Some(Box::new(ListNode {
                val: nums[(len + i - move_num as usize) % len],
                next: None,
            }));
            curr = &mut curr.as_mut()?.next;
        }

        res.unwrap().next
    }
}

// @lc code=end

fn main() -> Result<()> {
    let head: LinkedList = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: LinkedList = Solution::rotate_right(head.into(), k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
