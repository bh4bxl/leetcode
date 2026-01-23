// Created by bh4bxl at 2026/01/19 19:40
// leetgo: 1.4.16
// https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin
use rand::Rng;
use std::collections::{HashMap, HashSet};

struct RandomizedCollection {
    nums: Vec<i32>,
    indices: HashMap<i32, HashSet<usize>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {
    fn new() -> Self {
        Self {
            nums: Vec::new(),
            indices: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        let existed = self.indices.contains_key(&val);

        self.nums.push(val);
        let idx = self.nums.len() - 1;
        self.indices
            .entry(val)
            .or_insert(HashSet::new())
            .insert(idx);

        !existed
    }

    fn remove(&mut self, val: i32) -> bool {
        let idx = match self.indices.get(&val) {
            Some(set) => *set.iter().next().unwrap(),
            None => return false,
        };

        let last_idx = self.nums.len() - 1;
        let last_val = self.nums[last_idx];

        // Move last value to idx
        self.nums[idx] = last_val;

        // Remove idx from val's set
        {
            let set = self.indices.get_mut(&val).unwrap();
            set.remove(&idx);
            if set.is_empty() {
                self.indices.remove(&val);
            }
        }

        if idx != last_idx {
            let last_set = self.indices.get_mut(&last_val).unwrap();
            last_set.remove(&last_idx);
            last_set.insert(idx);
        }

        self.nums.pop();
        true
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        self.nums[rng.gen_range(0..self.nums.len())]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    #[allow(unused_mut)]
    let mut obj = RandomizedCollection::new();

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "insert" => {
                let method_params = split_array(&params[i])?;
                let val: i32 = deserialize(&method_params[0])?;
                let ans: bool = obj.insert(val).into();
                output.push(serialize(ans)?);
            }
            "remove" => {
                let method_params = split_array(&params[i])?;
                let val: i32 = deserialize(&method_params[0])?;
                let ans: bool = obj.remove(val).into();
                output.push(serialize(ans)?);
            }
            "getRandom" => {
                let ans: i32 = obj.get_random().into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
