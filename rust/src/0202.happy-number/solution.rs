// Created by bh4bxl at 2025/10/20 14:30
// leetgo: 1.4.15
// https://leetcode.com/problems/happy-number/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut squares_sums = vec![];

        fn get_squares_sum(n: i32) -> i32 {
            let mut nums = vec![];
            let mut m = n;
            while m > 0 {
                nums.push(m % 10);
                m /= 10;
            }
            let mut sum = 0;
            for i in nums {
                sum += i * i;
            }
            sum
        }

        let mut squares_sum = n;
        loop {
            if squares_sum == 1 {
                return true;
            }
            if squares_sums.contains(&squares_sum) {
                return false;
            }
            squares_sums.push(squares_sum.clone());
            squares_sum = get_squares_sum(squares_sum);
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_happy(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
