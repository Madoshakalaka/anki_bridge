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

use serde::{Deserialize, Serialize};

use crate::AnkiRequest;

/// Parameters for adding cards using the Add Cards dialog.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GuiAddCardsRequest {
    /// The note to add using the Add Cards dialog.
    pub note: GuiAddCardsNote,
}

/// A note for adding cards using the Add Cards dialog.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuiAddCardsNote {
    /// The deck name for the note.
    pub deck_name: String,
    /// The model name for the note.
    pub model_name: String,
    /// The fields of the note.
    pub fields: GuiAddCardsNoteFields,
    /// The tags of the note.
    pub tags: Vec<String>,
    /// The pictures attached to the note.
    pub picture: Vec<GuiAddCardsNotePicture>,
}

/// Fields of the note for adding cards using the Add Cards dialog.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiAddCardsNoteFields {
    /// The "Text" field of the note.
    #[serde(rename = "Text")]
    pub text: String,
    /// The "Extra" field of the note.
    #[serde(rename = "Extra")]
    pub extra: String,
}

/// Picture attached to the note for adding cards using the Add Cards dialog.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiAddCardsNotePicture {
    /// The URL of the picture.
    pub url: String,
    /// The filename of the picture.
    pub filename: String,
    /// The fields associated with the picture.
    pub fields: Vec<String>,
}

impl AnkiRequest for GuiAddCardsRequest {
    type Response = usize;

    const ACTION: &'static str = "guiAddCards";
    const VERSION: u8 = 6;
}
