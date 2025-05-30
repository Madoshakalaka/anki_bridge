/*
* The MIT License (MIT)
*
* Copyright (c) 2023 VaiTon <eyadlorenzo.issa@studio.unibo.it>
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::AnkiRequest;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NotesInfoRequest {
    pub notes: Vec<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesInfoResponse {
    pub note_id: u64,
    pub model_name: String,
    pub tags: Vec<String>,
    pub fields: HashMap<String, NotesInfoFieldsResponse>,
}
#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct NotesInfoFieldsResponse {
    pub value: String,
    pub order: usize,
}

impl AnkiRequest for NotesInfoRequest {
    type Response = Vec<NotesInfoResponse>;

    const ACTION: &'static str = "notesInfo";
    const VERSION: u8 = 6;
}
