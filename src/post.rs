use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    //pub post_id: i16,
    pub author_id: i16,
    pub header: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostForm {
    pub header: String,
    pub content: String,
}
