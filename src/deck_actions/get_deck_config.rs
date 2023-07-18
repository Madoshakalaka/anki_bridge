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
/// Parameters for the "`getDeckConfig`" action in `AnkiConnect`.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GetDeckConfigRequest {
    /// The name of the deck.
    pub deck: String,
}

/// Configuration options for a deck.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeckConfigResponse {
    /// Autoplay setting.
    pub autoplay: bool,
    /// Bury interday learning setting.
    pub bury_interday_learning: bool,
    #[serde(rename = "dyn")]
    /// Dyn setting.
    pub dyn_: bool,
    /// ID of the deck configuration.
    pub id: usize,
    /// Lapse configuration.
    pub lapse: GetDeckConfigLapse,
    /// Maximum taken cards setting.
    pub max_taken: usize,
    #[serde(rename = "mod")]
    /// Mod setting.
    pub mod_: usize,
    /// Name of the deck.
    pub name: String,
    /// New card configuration.
    pub new: GetDeckConfigNew,
    /// New gather priority setting.
    pub new_gather_priority: usize,
    /// New card mix setting.
    pub new_mix: usize,
    /// Minimum number of new cards per day setting.
    pub new_per_day_minimum: usize,
    /// New card sort order setting.
    pub new_sort_order: usize,
    /// Replay queue setting.
    pub replayq: bool,
    /// Review configuration.
    pub rev: GetDeckConfigRev,
    /// Review order setting.
    pub review_order: usize,
    /// Timer setting.
    pub timer: usize,
    /// Update sequence number.
    pub usn: isize,
}

/// Configuration options for new cards in a deck.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeckConfigNew {
    /// Bury setting.
    pub bury: bool,
    /// Delays between steps.
    pub delays: Vec<f32>,
    /// Initial factor setting.
    pub initial_factor: usize,
    /// Intervals between steps.
    pub ints: Vec<usize>,
    /// Order setting.
    pub order: usize,
    /// Number of new cards per day setting.
    pub per_day: usize,
}

/// Configuration options for lapsed cards in a deck.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeckConfigLapse {
    /// Delays between steps.
    pub delays: Vec<f32>,
    /// Leech action setting.
    pub leech_action: usize,
    /// Number of leech fails setting.
    pub leech_fails: usize,
    /// Minimum interval setting.
    pub min_int: usize,
    /// Interval multiplier setting.
    pub mult: f32,
}

/// Configuration options for review cards in a deck.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeckConfigRev {
    /// Bury setting.
    pub bury: bool,
    /// Ease factor for ease 4 setting.
    pub ease4: f32,
    /// Interval factor setting.
    pub ivl_fct: f32,
    /// Maximum interval setting.
    pub max_ivl: usize,
    /// Number of review cards per day setting.
    pub per_day: usize,
    /// Hard factor setting.
    pub hard_factor: f32,
}

impl AnkiRequest for GetDeckConfigRequest {
    type Response = GetDeckConfigResponse;

    const ACTION: &'static str = "getDeckConfig";
    const VERSION: u8 = 6;
}
