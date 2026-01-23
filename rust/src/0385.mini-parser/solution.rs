// Created by bh4bxl at 2026/01/19 20:44
// leetgo: 1.4.16
// https://leetcode.com/problems/mini-parser/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

// @lc code=begin

// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
impl NestedInteger {
    fn new_integer(num: i32) -> Self {
        NestedInteger::Int(num)
    }

    fn new_list() -> Self {
        NestedInteger::List(vec![])
    }

    fn add(&mut self, other: Self) {
        match self {
            NestedInteger::Int(_) => return,
            NestedInteger::List(li) => li.push(other),
        }
    }
}

impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        if !s.starts_with('[') {
            return NestedInteger::new_integer(s.parse::<i32>().unwrap());
        }

        let mut stack: Vec<NestedInteger> = vec![];
        let mut num = 0;
        let mut sign = 1;
        let mut in_num = false;

        for c in s.chars() {
            match c {
                '[' => stack.push(NestedInteger::new_list()),
                '-' => sign = -1,
                '0'..='9' => {
                    num = num * 10 + (c as i32 - '0' as i32);
                    in_num = true;
                }
                ',' | ']' => {
                    if in_num {
                        let ni = NestedInteger::new_integer(sign * num);
                        stack.last_mut().unwrap().add(ni);
                        num = 0;
                        sign = 1;
                        in_num = false;
                    }
                    if c == ']' && stack.len() > 1 {
                        let last = stack.pop().unwrap();
                        stack.last_mut().unwrap().add(last);
                    }
                }
                _ => {}
            }
        }

        stack.pop().unwrap()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: NestedInteger = Solution::deserialize(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
