use candid::CandidType;
use std::{
    hash::{Hash, Hasher},
    collections::HashSet,
};
use serde::{Serialize, Deserialize};
use types::{NobleId, TimestampMillis};

#[derive(Serialize, Deserialize, Default)]
pub struct FollowRequestMap {
    pub requests: HashSet<FollowRequest>,
}



#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct FollowRequest {
    pub sender: NobleId,
    pub receiver: NobleId,
    pub timestamp: TimestampMillis,
}

impl PartialEq for FollowRequest {
    fn eq(&self, other: &Self) -> bool {
        self.sender == other.sender && self.receiver == other.receiver
    }
}

impl Eq for FollowRequest {}

impl Hash for FollowRequest {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sender.hash(state);
        self.receiver.hash(state);
    }
}

impl FollowRequestMap {
    #[allow(dead_code)]
    pub fn does_request_exist(&self, request: &FollowRequest) -> bool {
        self.requests.contains(request)
    }

    #[allow(dead_code)]
    pub fn add_request(&mut self, request: &FollowRequest) -> bool {
        self.requests.insert(request.clone())
    }

    #[allow(dead_code)]
    pub fn remove_request(&mut self, request: &FollowRequest) -> bool {
        self.requests.remove(request)
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.requests.len()
    }
}