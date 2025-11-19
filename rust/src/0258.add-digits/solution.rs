// Created by bh4bxl at 2025/11/17 15:11
// leetgo: 1.4.15
// https://leetcode.com/problems/add-digits/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        fn get_digits_sum(num: i32) -> i32 {
            let mut num = num;
            let mut sum = 0;
            while num >= 10 {
                sum += num % 10;
                num /= 10;
            }

            sum + num
        }

        let mut sum = i32::MAX;
        let mut num = num;

        while sum >= 10 {
            sum = get_digits_sum(num);
            num = sum;
        }

        sum
    }
}

// @lc code=end

fn main() -> Result<()> {
    let num: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::add_digits(num).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
