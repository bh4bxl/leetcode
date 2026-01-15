// Created by bh4bxl at 2026/01/12 14:55
// leetgo: 1.4.16
// https://leetcode.com/problems/super-pow/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    const MOD: i32 = 1337;

    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        let mut res = 1;
        let a = a % Self::MOD;

        for digit in b {
            res = Self::pow_mod(res, 10);
            res = (res * Self::pow_mod(a, digit)) % Self::MOD;
        }

        res
    }

    fn pow_mod(mut base: i32, mut exp: i32) -> i32 {
        let mut res = 1;
        base %= Self::MOD;

        while exp > 0 {
            if exp & 1 == 1 {
                res = (res * base) % Self::MOD;
            }
            base = (base * base) % Self::MOD;
            exp >>= 1;
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let a: i32 = deserialize(&read_line()?)?;
    let b: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::super_pow(a, b).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
