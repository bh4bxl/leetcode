// Created by bh4bxl at 2025/07/31 15:56
// leetgo: 1.4.15
// https://leetcode.com/problems/permutation-sequence/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut nums = vec![];
        let mut res = String::from("");
        let mut factorial = 1;
        let mut k_m = k;

        for i in 0..n {
            nums.push(i + 1);
            if i != 0 {
                factorial *= i;
            }
        }
        factorial *= n;

        k_m -= 1;

        for _ in (1..n + 1).rev() {
            factorial /= nums.len() as i32;
            let group_num = (k_m / factorial) as usize;
            let num = nums.remove(group_num);
            k_m = k_m % factorial;
            res.push_str(&num.to_string());
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: String = Solution::get_permutation(n, k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
