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

/// Parameters for the "`getDeckStats`" action.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GetDeckStatsRequest {
    /// Names of the decks for which to retrieve statistics.
    pub decks: Vec<String>,
}

/// Statistics for a single deck.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDeckStatsResponse {
    /// ID of the deck.
    pub deck_id: usize,
    /// Name of the deck.
    pub name: String,
    /// Number of new cards in the deck.
    pub new_count: usize,
    /// Number of cards in the learning phase in the deck.
    pub learn_count: usize,
    /// Number of cards in the review phase in the deck.
    pub review_count: usize,
    /// Total number of cards in the deck.
    pub total_in_deck: usize,
}

impl AnkiRequest for GetDeckStatsRequest {
    type Response = HashMap<usize, GetDeckStatsResponse>;

    const ACTION: &'static str = "getDeckStats";
    const VERSION: u8 = 6;
}
