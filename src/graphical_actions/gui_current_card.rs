/*
* The MIT License (MIT)
*
* Copyright (c) 2023 Daniél Kerkmann <daniel@kerkmann.dev>
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

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::AnkiRequest;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GuiCurrentCardRequest;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the current card being reviewed.
pub struct GuiCurrentCardResponse {
    /// The answer of the current card.
    pub answer: String,
    /// The question of the current card.
    pub question: String,
    /// The deck name of the current card.
    pub deck_name: String,
    /// The model name of the current card.
    pub model_name: String,
    /// The field order of the current card.
    pub field_order: isize,
    /// The fields of the current card.
    pub fields: HashMap<String, GuiCurrentCardField>,
    /// The CSS of the current card.
    pub css: String,
    /// The template of the current card.
    pub template: String,
    /// The ID of the current card.
    pub card_id: usize,
    /// The buttons associated with the current card.
    pub buttons: Vec<isize>,
    /// The next reviews for the current card.
    pub next_reviews: Vec<String>,
}

/// Represents a face of the fields of the current card being reviewed.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiCurrentCardField {
    /// The value of the face.
    pub value: String,
    /// The order of the face.
    pub order: isize,
}

impl AnkiRequest for GuiCurrentCardRequest {
    type Response = GuiCurrentCardResponse;

    const ACTION: &'static str = "guiCurrentCard";
    const VERSION: u8 = 6;
}
