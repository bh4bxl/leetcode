// Created by bh4bxl at 2025/08/07 12:20
// leetgo: 1.4.15
// https://leetcode.com/problems/edit-distance/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        if word1.is_empty() && word2.is_empty() {
            return 0;
        }

        if word1.is_empty() {
            return word2.len() as i32;
        }
        if word2.is_empty() {
            return word1.len() as i32;
        }

        let (l1, l2) = (word1.len(), word2.len());
        let mut res = vec![vec![0; l2 + 1]; l1 + 1];
        for i in 0..=l1 {
            res[i][0] = i as i32;
        }
        for i in 0..=l2 {
            res[0][i] = i as i32;
        }

        for i in 1..=l1 {
            for j in 1..=l2 {
                let min_del = std::cmp::min(res[i - 1][j], res[i][j - 1]) + 1;
                let mut replace = res[i - 1][j - 1];
                if word1.chars().nth(i - 1) != word2.chars().nth(j - 1) {
                    replace += 1;
                }
                res[i][j] = std::cmp::min(min_del, replace);
            }
        }

        res[l1][l2]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let word1: String = deserialize(&read_line()?)?;
    let word2: String = deserialize(&read_line()?)?;
    let ans: i32 = Solution::min_distance(word1, word2).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
