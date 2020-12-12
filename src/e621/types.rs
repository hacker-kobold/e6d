use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Posts {
    pub posts: Vec<Post>,
}

#[derive(Deserialize, Debug)]
pub struct Post {
    pub id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub file: File,
    pub preview: PreviewFile,
    pub sample: SampleFile,
    pub score: Score,
    pub tags: Tags,
    pub locked_tags: Vec<String>,
    pub change_seq: u64,
    pub flags: Flags,
    pub rating: String,
    pub fav_count: u32,
    pub sources: Vec<String>,
    pub pools: Vec<u32>,
    pub relationships: Relationships,
    pub approver_id: Option<u32>,
    pub uploader_id: u32,
    pub description: String,
    pub comment_count: u32,
    pub is_favorited: bool,
    pub has_notes: bool,
    pub duration: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct File {
    pub width: u32,
    pub height: u32,
    pub ext: String,
    pub size: u64,
    pub md5: String,
    pub url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PreviewFile {
    pub width: u32,
    pub height: u32,
    pub url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SampleFile {
    pub has: bool,
    pub width: u32,
    pub height: u32,
    pub url: Option<String>,
    pub alternates: HashMap<String, AlternateFile>,
}

#[derive(Deserialize, Debug)]
pub struct AlternateFile {
    pub r#type: String,
    pub width: u32,
    pub height: u32,
    pub urls: Vec<Option<String>>,
}

#[derive(Deserialize, Debug)]
pub struct Score {
    pub up: i32,
    pub down: i32,
    pub total: i32,
}

#[derive(Deserialize, Debug)]
pub struct Tags {
    pub general: Vec<String>,
    pub species: Vec<String>,
    pub character: Vec<String>,
    pub copyright: Vec<String>,
    pub artist: Vec<String>,
    pub invalid: Vec<String>,
    pub lore: Vec<String>,
    pub meta: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Flags {
    pub pending: bool,
    pub flagged: bool,
    pub note_locked: bool,
    pub status_locked: bool,
    pub rating_locked: bool,
    pub deleted: bool,
}

#[derive(Deserialize, Debug)]
pub struct Relationships {
    pub parent_id: Option<u32>,
    pub has_children: bool,
    pub has_active_children: bool,
    pub children: Vec<u32>,
}
