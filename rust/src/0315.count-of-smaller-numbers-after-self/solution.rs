// Created by bh4bxl at 2025/12/09 14:25
// leetgo: 1.4.15
// https://leetcode.com/problems/count-of-smaller-numbers-after-self/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        fn get_index(sorted: &Vec<i32>, x: i32) -> usize {
            sorted.binary_search(&x).unwrap() + 1
        }

        fn update(bit: &mut Vec<i32>, mut i: usize) {
            while i < bit.len() {
                bit[i] += 1;
                i += i & (!i + 1);
            }
        }

        fn query(bit: &Vec<i32>, mut i: usize) -> i32 {
            let mut sum = 0;
            while i > 0 {
                sum += bit[i];
                i &= i - 1;
            }
            sum
        }

        let n = nums.len();
        let mut sorted = nums.clone();

        sorted.sort();
        sorted.dedup();

        let mut bit = vec![0; sorted.len() + 1];
        let mut res = vec![0; n];

        for i in (0..n).rev() {
            let idx = get_index(&sorted, nums[i]);
            res[i] = query(&bit, idx - 1);
            update(&mut bit, idx);
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::count_smaller(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
