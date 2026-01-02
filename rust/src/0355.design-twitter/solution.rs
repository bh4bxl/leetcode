// Created by bh4bxl at 2025/12/31 23:11
// leetgo: 1.4.16
// https://leetcode.com/problems/design-twitter/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin
use std::collections::{BinaryHeap, HashMap, HashSet};

struct Tweet {
    time: i32,
    id: i32,
}

struct Twitter {
    time: i32,
    tweets: HashMap<i32, Vec<Tweet>>,
    followees: HashMap<i32, HashSet<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        Self {
            time: 0,
            tweets: HashMap::new(),
            followees: HashMap::new(),
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.time += 1;
        self.tweets.entry(user_id).or_default().push(Tweet {
            time: self.time,
            id: tweet_id,
        });
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::new();

        // Add user's own tweets
        if let Some(tw) = self.tweets.get(&user_id) {
            for t in tw.iter().rev().take(10) {
                heap.push((t.time, t.id));
            }
        }

        // Add followees' tweets
        if let Some(following) = self.followees.get(&user_id) {
            for &fid in following {
                if let Some(tw) = self.tweets.get(&fid) {
                    for t in tw.iter().rev().take(10) {
                        heap.push((t.time, t.id));
                    }
                }
            }
        }

        // Extract top 10
        let mut res = vec![];
        while let Some((_time, id)) = heap.pop() {
            res.push(id);
            if res.len() == 10 {
                break;
            }
        }

        res
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id == followee_id {
            return;
        }

        self.followees
            .entry(follower_id)
            .or_default()
            .insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(set) = self.followees.get_mut(&follower_id) {
            set.remove(&followee_id);
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    #[allow(unused_mut)]
    let mut obj = Twitter::new();

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "postTweet" => {
                let method_params = split_array(&params[i])?;
                let user_id: i32 = deserialize(&method_params[0])?;
                let tweet_id: i32 = deserialize(&method_params[1])?;
                obj.post_tweet(user_id, tweet_id);
                output.push("null".to_string());
            }
            "getNewsFeed" => {
                let method_params = split_array(&params[i])?;
                let user_id: i32 = deserialize(&method_params[0])?;
                let ans: Vec<i32> = obj.get_news_feed(user_id).into();
                output.push(serialize(ans)?);
            }
            "follow" => {
                let method_params = split_array(&params[i])?;
                let follower_id: i32 = deserialize(&method_params[0])?;
                let followee_id: i32 = deserialize(&method_params[1])?;
                obj.follow(follower_id, followee_id);
                output.push("null".to_string());
            }
            "unfollow" => {
                let method_params = split_array(&params[i])?;
                let follower_id: i32 = deserialize(&method_params[0])?;
                let followee_id: i32 = deserialize(&method_params[1])?;
                obj.unfollow(follower_id, followee_id);
                output.push("null".to_string());
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
