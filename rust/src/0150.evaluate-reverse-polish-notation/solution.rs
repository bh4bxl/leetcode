// Created by bh4bxl at 2025/09/24 14:53
// leetgo: 1.4.15
// https://leetcode.com/problems/evaluate-reverse-polish-notation/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut nums = vec![];

        for s in tokens {
            match s.parse::<i32>() {
                Ok(num) => nums.push(num),
                Err(_e) => {
                    let num2 = nums.pop().unwrap();
                    let num1 = nums.pop().unwrap();
                    let mut num = 0;
                    if s == "+" {
                        num = num1 + num2;
                    } else if s == "-" {
                        num = num1 - num2;
                    } else if s == "*" {
                        num = num1 * num2;
                    } else if s == "/" {
                        num = num1 / num2;
                    }
                    nums.push(num);
                }
            }
        }

        nums[0]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let tokens: Vec<String> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::eval_rpn(tokens).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
