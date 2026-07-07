//! https://leetcode.com/problems/design-twitter/
//!
//! A tiny Twitter: `post_tweet`, `follow`, `unfollow`, and `get_news_feed`
//! (10 most recent tweet ids from the user and everyone they follow).

pub struct Twitter {
    tweets: Vec<(i32, i32, i32)>, // (timestamp, user_id, tweet_id)
    following: std::collections::HashMap<i32, std::collections::HashSet<i32>>,
    clock: i32,
}

impl Twitter {
    pub fn new() -> Self {
        todo!()
    }

    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let _ = (user_id, tweet_id);
        todo!()
    }

    pub fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let _ = user_id;
        todo!()
    }

    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        let _ = (follower_id, followee_id);
        todo!()
    }

    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        let _ = (follower_id, followee_id);
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feed_includes_followees() {
        let mut tw = Twitter::new();
        tw.post_tweet(1, 5);
        assert_eq!(tw.get_news_feed(1), vec![5]);
        tw.follow(1, 2);
        tw.post_tweet(2, 6);
        assert_eq!(tw.get_news_feed(1), vec![6, 5]);
        tw.unfollow(1, 2);
        assert_eq!(tw.get_news_feed(1), vec![5]);
    }
}
