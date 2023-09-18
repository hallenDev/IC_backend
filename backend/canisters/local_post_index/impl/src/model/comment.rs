use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use types::{
    TimestampMillis, NobleId, CommentId, CommentDetail
};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Comment {
    pub noble_id: NobleId,
    pub description: String,
    pub liked_users: HashSet<NobleId>,
    pub comments_count: u32,
    pub children_count: u32,
    pub parent: Option<CommentId>,
    pub prev_sibling: Option<CommentId>,
    pub next_sibling: Option<CommentId>,
    pub first_child: Option<CommentId>,
    pub last_child: Option<CommentId>,
    pub date_created: TimestampMillis,
    pub is_alive: bool,
}

impl Comment {
    pub fn new(
        noble_id: NobleId,
        description: String,
        now: TimestampMillis,
    ) -> Self {
        Comment {
            noble_id,
            description,
            liked_users: HashSet::new(),
            comments_count: 0,
            children_count: 0,
            parent: None,
            prev_sibling: None,
            next_sibling: None,
            first_child: None,
            last_child: None,
            date_created: now,
            is_alive: true,
        }
    }

    pub fn can_show(&self, block_me_users: &Vec<NobleId>) -> bool {
        !block_me_users.contains(&self.noble_id)
    }

    pub fn to_detail(&self, comment_id: CommentId, noble_id: NobleId) -> CommentDetail {
        CommentDetail {
            noble_id: self.noble_id,
            comment_id,
            parent_comment_id: self.parent.unwrap(),
            description: self.description.clone(),
            liked_users_count: self.liked_users.len() as u32,
            comments_count: self.comments_count,
            date_created: self.date_created,
            like_state: self.liked_users.contains(&noble_id),
            loading_like: false,
            loading_delete: false,
        }
    }

    pub fn clear(&mut self) {
        self.noble_id = 0;
        self.description.clear();
        self.liked_users.clear();
        self.comments_count = 0;
        self.children_count = 0;
        self.parent = None;
        self.prev_sibling = None;
        self.next_sibling = None;
        self.first_child = None;
        self.last_child = None;
        self.date_created = 0;
        self.is_alive = false;
    }
}