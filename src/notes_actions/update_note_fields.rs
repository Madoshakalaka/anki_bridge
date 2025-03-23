use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::AnkiRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNoteFieldsRequest {
    pub note: UpdateNoteFields,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNoteFields {
    pub id: usize,
    pub fields: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Vec<NoteMedia>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Vec<NoteMedia>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<Vec<NoteMedia>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteMedia {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_hash: Option<String>,
    pub fields: Vec<String>,
}


impl AnkiRequest for UpdateNoteFieldsRequest {
    type Response = ();

    const ACTION: &'static str = "updateNoteFields";
    const VERSION: u8 = 6;
}

