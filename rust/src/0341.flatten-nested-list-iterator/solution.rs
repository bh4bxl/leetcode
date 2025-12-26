// Created by bh4bxl at 2025/12/22 21:54
// leetgo: 1.4.15
// https://leetcode.com/problems/flatten-nested-list-iterator/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
struct NestedIterator {
    stack: Vec<NestedInteger>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut stack = vec![];

        // Push in reverse order
        for i in nested_list.into_iter().rev() {
            stack.push(i);
        }

        Self { stack }
    }

    fn next(&mut self) -> i32 {
        if let Some(NestedInteger::Int(val)) = self.stack.pop() {
            val
        } else {
            0
        }
    }

    fn has_next(&mut self) -> bool {
        while let Some(top) = self.stack.last() {
            match top {
                NestedInteger::Int(_) => return true,
                NestedInteger::List(_) => {
                    if let Some(NestedInteger::List(list)) = self.stack.pop() {
                        for i in list.into_iter().rev() {
                            self.stack.push(i);
                        }
                    }
                }
            }
        }

        false
    }
}

// @lc code=end

// Warning: this is a manual question, the generated test code may be incorrect.
fn main() -> Result<()> {
    let nested_list: Vec<NestedInteger> = deserialize(&read_line()?)?;
    let ans: i32 = Solution(nested_list).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
