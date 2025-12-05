// Created by bh4bxl at 2025/12/01 21:40
// leetgo: 1.4.15
// https://leetcode.com/problems/remove-invalid-parentheses/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        fn is_valid(s: &str) -> bool {
            let mut cnt = 0;
            for c in s.chars() {
                match c {
                    '(' => cnt += 1,
                    ')' => {
                        if cnt == 0 {
                            return false;
                        } else {
                            cnt -= 1;
                        }
                    }
                    _ => {}
                }
            }
            cnt == 0
        }

        use std::collections::{HashSet, VecDeque};

        let mut res = vec![];
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(s.clone());
        queue.push_back(s);

        let mut found = false;

        while let Some(curr) = queue.pop_front() {
            if is_valid(&curr) {
                res.push(curr.clone());
                found = true;
            }

            if found {
                continue;
            }

            for i in 0..curr.len() {
                let c = curr.chars().nth(i).unwrap();
                if c != '(' && c != ')' {
                    continue;
                }
                let next = format!("{}{}", &curr[0..i], &curr[i + 1..]);
                if visited.insert(next.clone()) {
                    queue.push_back(next);
                }
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: Vec<String> = Solution::remove_invalid_parentheses(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
