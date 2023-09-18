use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use types::{
    TimestampMillis, PostId, Category, NobleId, PostPrivacy, FileId, PostDetail, CommentId, CommentDetail
};

use super::comment::Comment;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Post {
    pub post_id: PostId,
    pub noble_id: NobleId,
    pub title: String,
    pub description: String,
    pub category: Category,
    pub link_url: String,
    pub video_url: String,
    pub attached_file_id: FileId,

    pub comments: Vec<Comment>,
    
    pub liked_users: HashSet<NobleId>,
    pub contributed_users: HashSet<NobleId>,
    pub post_privacy: PostPrivacy,
    pub invited_users: HashSet<NobleId>,

    pub date_created: TimestampMillis,
    pub date_updated: TimestampMillis,
    pub date_last_commented: TimestampMillis,
}

impl Post {
    pub fn new(
        post_id: PostId,
        noble_id: NobleId,
        title: String,
        description: String,
        category: Category,
        link_url: String,
        video_url: String,
        attached_file_id: FileId,
        post_privacy: PostPrivacy,
        invited_users: HashSet<NobleId>,
        now: TimestampMillis,
    ) -> Self {
        Post {
            post_id,
            noble_id,
            title,
            description,
            category,

            link_url,
            video_url,
            attached_file_id,

            comments: vec![Comment::new(noble_id, String::new(), now)],

            post_privacy,
            invited_users,
            liked_users: HashSet::default(),
            contributed_users: HashSet::default(),

            date_created: now,
            date_updated: now,
            date_last_commented: now,
        }
    }

    pub fn can_show(
        &self,
        noble_id: NobleId,
        following_list: &Vec<NobleId>,
        block_me_users: &Vec<NobleId>,
    ) -> bool {
        if self.noble_id == noble_id {
            return true;
        }

        if block_me_users.contains(&self.noble_id) {
            return false;
        }
    
        self.post_privacy == PostPrivacy::Everyone ||
        (self.post_privacy == PostPrivacy::Followers && following_list.contains(&self.noble_id)) ||
        (self.post_privacy == PostPrivacy::SpecificUsers && self.invited_users.contains(&noble_id))
    }

    pub fn get_sub_comments(&self, comment_id: CommentId, noble_id: NobleId, from: u32, limit: u32, block_me_users: &Vec<NobleId>) -> (Vec<CommentDetail>, bool) {
        if let Some(comment) = self.comments.get(comment_id as usize) {
            let mut comments = vec![];
            let mut cnt = 0;
            let mut id = comment.first_child;

            loop {
                if id.is_none() {
                    break;
                }
                if let Some(comment) = self.comments.get(id.unwrap() as usize) {
                    if !comment.can_show(&block_me_users) {
                        id = comment.next_sibling;
                        continue;
                    }
                    if cnt >= from + limit {
                        return (comments, true);
                    }
                    if from <= cnt && cnt < from + limit {
                        comments.push(comment.to_detail(id.unwrap(), noble_id));
                    }
                    cnt += 1;
                    id = comment.next_sibling;
                }
            }
            (comments, false)
        } else {
            (vec![], false)
        }
    }

    pub fn to_detail(&self, like_state: bool, bookmark_state: bool) -> PostDetail {
        PostDetail {
            post_id: self.post_id,
            noble_id: self.noble_id,
            title: self.title.clone(),
            description: self.description.clone(),
            category: self.category,
            link_url: self.link_url.clone(),
            video_url: self.video_url.clone(),
            attached_file_id: self.attached_file_id,
            liked_users_count: self.liked_users.len() as u32,
            comments_count: self.comments.get(0).unwrap().comments_count,
            date_created: self.date_created,
            date_updated: self.date_updated,
            date_last_commented: self.date_last_commented,
            like_state,
            bookmark_state,
        }
    }

    fn get_comment_id_for_new(&mut self) -> Option<CommentId> {
        match self.comments.iter().position(|item| item.is_alive == false) {
            Some(id) => Some(id as CommentId),
            None => {
                self.comments.push(Comment::default());
                Some((self.comments.len() - 1) as CommentId)
            }
        }
    }

    fn update_parent(&mut self, comment_id: Option<CommentId>, value: u32) {
        if comment_id.is_none() {
            return;
        }
        if let Some(comment) = self.comments.get_mut(comment_id.unwrap() as usize) {
            comment.comments_count += value;
            let parent = comment.parent;
            self.update_parent(parent, value);
        }
    }

    pub fn add_comment(
        &mut self,
        noble_id: NobleId,
        comment_id: CommentId,
        description: String,
        now: TimestampMillis,
    ) -> Option<CommentId> {
        let new_comment_id = self.get_comment_id_for_new();

        if let Some(comment) = self.comments.get_mut(comment_id as usize) {
            let last_child_id = comment.last_child;
            comment.last_child = new_comment_id;
            comment.children_count += 1;
            
            match last_child_id {
                Some(id) => {
                    if let Some(last_child) = self.comments.get_mut(id as usize) {
                        last_child.next_sibling = new_comment_id;
                    }
                },
                None => comment.first_child = new_comment_id,
            }
            if let Some(new_comment) = self.comments.get_mut(new_comment_id.unwrap() as usize) {
                new_comment.noble_id = noble_id;
                new_comment.description = description;
                new_comment.parent = Some(comment_id);
                new_comment.prev_sibling = last_child_id;
                new_comment.date_created = now;
                new_comment.is_alive = true;
            }
            self.date_last_commented = now;
            self.update_parent(Some(comment_id), 1);
            return new_comment_id;
        }
        None
    }

    pub fn remove_comment(&mut self, comment_id: CommentId) -> bool {
        if let Some(comment) = self.comments.get_mut(comment_id as usize) {
            if comment.is_alive == false {
                return false;
            }
            let parent_id = comment.parent;
            let prev_sibling_id = comment.prev_sibling;
            let next_sibling_id = comment.next_sibling;
            let first_child_id = comment.first_child;
            comment.clear();

            match prev_sibling_id {
                Some(id) => {
                    if let Some(prev_sibling) = self.comments.get_mut(id as usize) {
                        prev_sibling.next_sibling = next_sibling_id;
                    }
                },
                None => {
                    if let Some(parent) = self.comments.get_mut(parent_id.unwrap() as usize) {
                        parent.first_child = next_sibling_id;
                    }
                }
            }

            match next_sibling_id {
                Some(id) => {
                    if let Some(next_sibling) = self.comments.get_mut(id as usize) {
                        next_sibling.prev_sibling = prev_sibling_id;
                    }
                },
                None => {
                    if let Some(parent) = self.comments.get_mut(parent_id.unwrap() as usize) {
                        parent.last_child = prev_sibling_id;
                    }
                }
            }

            let value = self.remove_all_sub_comments(first_child_id) + 1;

            self.update_parent(parent_id, 0-value);
            if let Some(parent) = self.comments.get_mut(parent_id.unwrap() as usize) {
                parent.children_count -= 1;
            }
            return true;
        }
        return false;
    }

    fn remove_all_sub_comments(&mut self, comment_id: Option<CommentId>) -> u32 {
        if comment_id.is_none() {
            return 0;
        }
        let mut result = 1;
        if let Some(comment) = self.comments.get_mut(comment_id.unwrap() as usize) {
            if comment.is_alive == false {
                assert!(false);
            }
            let first_child = comment.first_child;
            let next_sibling = comment.next_sibling;
            comment.clear();
            result += self.remove_all_sub_comments(first_child);
            result += self.remove_all_sub_comments(next_sibling);
        }
        return result;
    }

    pub fn edit_comment(&mut self, noble_id: NobleId, comment_id: CommentId, description: String) -> bool {
        if let Some(comment) = self.comments.get_mut(comment_id as usize) {
            if comment.noble_id == noble_id {
                comment.description = description;
                return true;
            }
        }
        false
    }

    #[cfg(test)]
    pub fn validate_state(&self) -> bool {

        #[derive(Debug, Default)]
        struct State {
            pub comments_count: u32,
            pub children_count: u32,
            pub is_visited: bool,
        }
        let mut states = Vec::with_capacity(self.comments.len());
        for _ in 0..self.comments.len() {
            states.push(State::default());
        }

        fn dfs(post: &Post, comment_id: Option<CommentId>, states: &mut Vec<State>) {
            if comment_id.is_none() {
                return;
            }
            if let Some(comment) = post.comments.get(comment_id.unwrap() as usize) {
                dfs(post, comment.first_child, states);
                dfs(post, comment.next_sibling, states);
                let state = states.get_mut(comment_id.unwrap() as usize).unwrap();
                state.is_visited = true;
                let comments_count = state.comments_count;

                if comment.parent.is_some() {
                    if let Some(parent_state) = states.get_mut(comment.parent.unwrap() as usize) {
                        parent_state.children_count += 1;
                        parent_state.comments_count += 1 + comments_count;
                    }
                }
            }
        }

        dfs(self, Some(0), &mut states);

        for i in 0..self.comments.len() {
            let comment = self.comments.get(i).unwrap();
            let state = states.get(i).unwrap();

            if comment.comments_count != state.comments_count ||
               comment.children_count != state.children_count ||
               comment.is_alive != state.is_visited {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    #[allow(dead_code)]
    fn total_test() {
        let mut post = Post::default();
        post.comments.push(Comment::new(1, "".to_string(), 0));
        
        post.add_comment(1, 0, "".to_string(), 0);
        post.add_comment(1, 1, "".to_string(), 0);
        post.add_comment(1, 0, "".to_string(), 0);
        post.add_comment(1, 0, "".to_string(), 0);
        post.add_comment(1, 0, "".to_string(), 0);
        post.add_comment(1, 4, "".to_string(), 0);
        post.add_comment(1, 4, "".to_string(), 0);
        post.add_comment(1, 4, "".to_string(), 0);
        post.add_comment(1, 7, "".to_string(), 0);
        post.add_comment(1, 7, "".to_string(), 0);
        assert_eq!(post.validate_state(), true);
        post.remove_comment(2);
        assert_eq!(post.validate_state(), true);
        post.remove_comment(1);
        assert_eq!(post.validate_state(), true);
        post.remove_comment(4);
        assert_eq!(post.validate_state(), true);
        post.remove_comment(5);
        assert_eq!(post.validate_state(), true);
    }
}