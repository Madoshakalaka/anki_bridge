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

use serde::Serialize;
use serde_tuple::Serialize_tuple;

use crate::AnkiRequest;

/// Parameters for the "insertReviews" action.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InsertReviewsRequest {
    #[serde(flatten)]
    data: Vec<InsertReviewsData>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize_tuple)]
pub struct InsertReviewsData {
    review_time: usize,
    card_id: usize,
    usn: isize,
    button_pressed: usize,
    new_interval: isize,
    previous_interval: isize,
    new_factor: usize,
    review_duration: usize,
    review_type: usize,
}

impl AnkiRequest for InsertReviewsRequest {
    type Response = ();

    const ACTION: &'static str = "insertReviews";
    const VERSION: u8 = 6;
}
