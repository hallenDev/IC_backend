use serde::{Serialize, Deserialize};

mod lifecycle;
mod queries;
mod updates;

pub use lifecycle::*;
pub use queries::*;
pub use updates::*;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    NewPost(Box<NewPost>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewPost {
}