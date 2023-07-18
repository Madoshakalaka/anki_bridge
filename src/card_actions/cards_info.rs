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

/// Parameters for retrieving information about cards.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CardsInfoRequest {
    /// The list of card IDs.
    pub cards: Vec<usize>,
}

/// Represents the information about a card.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardsInfoResponse {
    /// The answer side of the card.
    pub answer: String,
    /// The question side of the card.
    pub question: String,
    /// The name of the deck the card belongs to.
    pub deck_name: String,
    /// The name of the model (note type) of the card.
    pub model_name: String,
    /// The order of the fields.
    pub field_order: usize,
    /// The fields of the card.
    pub fields: HashMap<String, CardsInfoField>,
    /// The CSS style applied to the card.
    pub css: String,
    /// The ID of the card.
    pub card_id: usize,
    /// The interval of the card.
    pub interval: usize,
    /// The ID of the note that the card belongs to.
    pub note: usize,
    /// The ordinal value of the card.
    pub ord: usize,
    /// The type of the card.
    #[serde(rename = "type")]
    pub type_field: usize,
    /// The queue of the card.
    pub queue: usize,
    /// The due date of the card.
    pub due: usize,
    /// The number of repetitions of the card.
    pub reps: usize,
    /// The number of lapses of the card.
    pub lapses: usize,
    /// The number of cards left in the card's queue.
    pub left: usize,
    /// The modification time of the card.
    #[serde(rename = "mod")]
    pub mod_: usize,
}

/// Represents the facade of a card.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CardsInfoField {
    /// The value of the facade.
    pub value: String,
    /// The order of the facade.
    pub order: usize,
}

impl AnkiRequest for CardsInfoRequest {
    type Response = CardsInfoResponse;

    const ACTION: &'static str = "cardsInfo";
    const VERSION: u8 = 6;
}
