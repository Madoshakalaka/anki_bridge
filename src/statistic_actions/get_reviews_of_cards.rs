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

/// Parameters for the "`getReviewsOfCards`" action.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GetReviewsOfCardsRequest {
    /// IDs of the cards to get reviews.
    pub cards: Vec<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReviewsOfCardsResponse {
    pub id: u64,
    pub usn: i64,
    pub ease: usize,
    pub ivl: isize,
    pub last_ivl: isize,
    pub factor: usize,
    pub time: usize,
    pub r#type: usize,
}

impl AnkiRequest for GetReviewsOfCardsRequest {
    type Response = HashMap<String, Vec<GetReviewsOfCardsResponse>>;

    const ACTION: &'static str = "getReviewsOfCards";
    const VERSION: u8 = 6;
}
