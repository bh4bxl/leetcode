// Created by bh4bxl at 2025/09/16 12:58
// leetgo: 1.4.15
// https://leetcode.com/problems/candy/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        if ratings.is_empty() {
            return 0;
        }

        let n = ratings.len();
        let mut candies = vec![1; n];

        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }

        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                candies[i] = candies[i].max(candies[i + 1] + 1);
            }
        }

        candies.iter().sum()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ratings: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::candy(ratings).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
