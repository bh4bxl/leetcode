// Created by bh4bxl at 2026/01/13 17:44
// leetgo: 1.4.16
// https://leetcode.com/problems/guess-number-higher-or-lower/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let (mut left, mut right) = (1, n);

        while left <= right {
            let mid = left + (right - left) / 2;
            match guess(mid) {
                0 => return mid,
                -1 => right = mid - 1,
                1 => left = mid + 1,
                _ => unreachable!(),
            }
        }

        -1
    }
}

// @lc code=end

// Warning: this is a manual question, the generated test code may be incorrect.
fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let pick: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::guess_number(n, pick).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
