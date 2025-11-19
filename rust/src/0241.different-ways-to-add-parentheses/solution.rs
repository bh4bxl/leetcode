// Created by bh4bxl at 2025/11/17 12:34
// leetgo: 1.4.15
// https://leetcode.com/problems/different-ways-to-add-parentheses/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        fn compute(expr: &str) -> Vec<i32> {
            let mut res = Vec::new();

            for (i, c) in expr.char_indices() {
                if c == '+' || c == '-' || c == '*' {
                    let left = compute(&expr[..i]);
                    let right = compute(&expr[i + 1..]);

                    for &l in &left {
                        for &r in &right {
                            let val = match c {
                                '+' => l + r,
                                '-' => l - r,
                                '*' => l * r,
                                _ => unreachable!(),
                            };
                            res.push(val);
                        }
                    }
                }
            }

            if res.is_empty() {
                res.push(expr.parse::<i32>().unwrap());
            }

            res
        }

        compute(&expression)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let expression: String = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::diff_ways_to_compute(expression).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
