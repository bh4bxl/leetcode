// Created by bh4bxl at 2025/10/07 15:03
// leetgo: 1.4.15
// https://leetcode.com/problems/fraction-to-recurring-decimal/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let mut res = "".to_string();

        if numerator > 0 && denominator < 0 || numerator < 0 && denominator > 0 {
            res.push('-');
        }
        let (a, b) = ((numerator as i64).abs(), (denominator as i64).abs());
        let digit = a / b;
        res.push_str(&(digit.to_string()));
        let mut remain = a % b;
        if remain == 0 {
            return res;
        }

        res.push('.');
        let mut count = 1;
        let mut map = std::collections::HashMap::new();

        while remain != 0 && !map.contains_key(&(remain * 10)) {
            remain *= 10;
            count += 1;
            map.insert(remain, count.clone());
            res.push_str(&(remain / b).to_string());
            remain %= b;
        }
        if remain != 0 {
            res.push(')');
            let val = map.get(&(remain * 10)).unwrap();
            let point = res.find('.').unwrap();
            res.insert(*val + point - 1, '(');
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let numerator: i32 = deserialize(&read_line()?)?;
    let denominator: i32 = deserialize(&read_line()?)?;
    let ans: String = Solution::fraction_to_decimal(numerator, denominator).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
