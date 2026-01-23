// Created by bh4bxl at 2026/01/19 20:32
// leetgo: 1.4.16
// https://leetcode.com/problems/shuffle-an-array/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin
use rand::seq::SliceRandom;

struct Solution {
    org: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self { org: nums.clone() }
    }

    fn reset(&self) -> Vec<i32> {
        self.org.clone()
    }

    fn shuffle(&self) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let mut res = self.org.clone();

        res.shuffle(&mut rng);

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    let constructor_params = split_array(&params[0])?;
    let nums: Vec<i32> = deserialize(&constructor_params[0])?;
    #[allow(unused_mut)]
    let mut obj = Solution::new(nums);

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "reset" => {
                let ans: Vec<i32> = obj.reset().into();
                output.push(serialize(ans)?);
            }
            "shuffle" => {
                let ans: Vec<i32> = obj.shuffle().into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
