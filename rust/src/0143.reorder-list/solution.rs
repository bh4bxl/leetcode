// Created by bh4bxl at 2025/09/22 10:51
// leetgo: 1.4.15
// https://leetcode.com/problems/reorder-list/

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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut curr = head.as_ref();
        let mut list = vec![];

        while let Some(n) = curr {
            let v = n.val;
            list.push(v);
            curr = n.next.as_ref();
        }

        let mut dummy = ListNode { val: 0, next: None };
        let (mut i, mut j) = (0, list.len().saturating_sub(1));
        let mut tail = &mut dummy;

        while i <= j {
            if i == j {
                tail.next = Some(Box::new(ListNode {
                    val: list[i],
                    next: None,
                }));
                break;
            }
            tail.next = Some(Box::new(ListNode {
                val: list[i],
                next: None,
            }));
            tail = tail.next.as_mut().unwrap();

            tail.next = Some(Box::new(ListNode {
                val: list[j],
                next: None,
            }));
            tail = tail.next.as_mut().unwrap();
            i += 1;
            j = j.saturating_sub(1);
        }

        *head = dummy.next
    }
}

// @lc code=end

fn main() -> Result<()> {
    let head: LinkedList = deserialize(&read_line()?)?;
    Solution::reorder_list(head.into());
    let ans: LinkedList = head.into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
