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

use crate::{anki_connect_send, Result};

/// Gets the complete list of deck names for the current user.
pub fn deck_names() -> Result<Vec<String>> {
    anki_connect_send::<(), _>("deckNames", None)
}

/// Gets the complete list of deck names for the current user.
pub fn deck_name_and_ids() -> Result<HashMap<String, usize>> {
    anki_connect_send::<(), _>("deckNamesAndIds", None)
}

/// Parameters for the "getDecks" action in AnkiConnect.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDecksParams {
    /// Array of card IDs.
    pub cards: Vec<usize>,
}

/// Accepts an array of card IDs and returns an object with each deck name as a key, and its value an array of the given cards which belong to it.
pub fn get_decks(params: GetDecksParams) -> Result<HashMap<String, Vec<usize>>> {
    anki_connect_send("getDecks", Some(params))
}

/// Parameters for the "createDeck" action in AnkiConnect.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDeckParams {
    /// The name of the new deck.
    pub deck: String,
}

/// Create a new empty deck. Will not overwrite a deck that exists with the same name.
pub fn create_deck(params: CreateDeckParams) -> Result<usize> {
    anki_connect_send("createDeck", Some(params))
}

/// Parameters for the "changeDeck" action in AnkiConnect.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ChangeDeckParams {
    /// IDs of the cards to move.
    pub cards: Vec<usize>,

    /// The name of the target deck.
    pub deck: String,
}

/// Moves cards with the given IDs to a different deck, creating the deck if it doesn’t exist yet.
pub fn change_deck(params: ChangeDeckParams) -> Result<()> {
    anki_connect_send("changeDeck", Some(params))
}

/// Parameters for the "deleteDecks" action in AnkiConnect.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDecksParams {
    /// Names of the decks to delete.
    pub decks: Vec<String>,

    /// Indicates whether to also delete the cards in the decks.
    pub cards_too: bool,
}

/// Deletes decks with the given names. The argument [cards_too](DeleteDecksParams::cards_too) must be specified and set to true.
pub fn delete_decks(params: DeleteDecksParams) -> Result<()> {
    anki_connect_send("deleteDecks", Some(params))
}

/// Parameters for the "getDeckConfig" action in AnkiConnect.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeckConfigParams {
    /// The name of the deck.
    pub deck: String,
}

/// Configuration options for a deck.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeckConfig {
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

/// Gets the configuration group object for the given deck.
pub fn get_deck_config(params: GetDeckConfigParams) -> Result<GetDeckConfig> {
    anki_connect_send("getDeckConfig", Some(params))
}

/// Parameters for saving a deck configuration.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveDeckConfigParams {
    /// The deck configuration to save.
    pub config: SaveDeckConfig,
}

/// Configuration options for saving a deck configuration.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveDeckConfig {
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
    pub lapse: SaveDeckConfigLapse,
    /// Maximum taken cards setting.
    pub max_taken: usize,
    #[serde(rename = "mod")]
    /// Mod setting.
    pub mod_: usize,
    /// Name of the deck.
    pub name: String,
    /// New card configuration.
    pub new: SaveDeckConfigNew,
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
    pub rev: SaveDeckConfigRev,
    /// Review order setting.
    pub review_order: usize,
    /// Timer setting.
    pub timer: usize,
    /// Update sequence number.
    pub usn: isize,
}

/// Configuration options for saving new cards in a deck.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveDeckConfigNew {
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

/// Configuration options for saving lapsed cards in a deck.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveDeckConfigLapse {
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

/// Configuration options for saving review cards in a deck.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveDeckConfigRev {
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

/// Saves the given configuration group, returning [true] on success or [false] if the ID of the configuration group is invalid (such as when it does not exist).
pub fn save_deck_config(params: SaveDeckConfigParams) -> Result<bool> {
    anki_connect_send("saveDeckConfig", Some(params))
}

/// Parameters for changing the configuration group of decks by ID.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveDeckConfigIdParams {
    /// Names of the decks to change the configuration group for.
    pub decks: Vec<String>,
    /// ID of the configuration group to set.
    pub config_id: usize,
}

/// Changes the configuration group for the given decks to the one with the given ID. Returns
/// [true] on success or [false] if the given configuration group or any of the given decks do not exist.
pub fn save_deck_config_id(params: SaveDeckConfigIdParams) -> Result<bool> {
    anki_connect_send("setDeckConfigId", Some(params))
}

/// Parameters for cloning a deck configuration by ID.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloneDeckConfigIdParams {
    /// Name of the new configuration group.
    pub name: String,
    /// ID of the configuration group to clone from.
    pub clone_from: usize,
}

/// Creates a new configuration group with the given name, cloning from the group with the given
/// ID, or from the default group if this is unspecified. Returns the ID of the new configuration
/// group, or [false] if the specified group to clone from does not exist.
pub fn clone_deck_config_id(params: CloneDeckConfigIdParams) -> Result<usize> {
    anki_connect_send("cloneDeckConfigId", Some(params))
}

/// Parameters for removing a configuration group by ID.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveDeckConfigIdParams {
    /// ID of the configuration group to remove.
    pub config_id: usize,
}

/// Removes the configuration group with the given ID, returning [true] if successful, or [false] if attempting to remove either the default configuration group (ID = 1) or a configuration group that does not exist.
pub fn remove_deck_config_id(params: RemoveDeckConfigIdParams) -> Result<bool> {
    anki_connect_send("removeDeckConfigId", Some(params))
}

/// Parameters for the "getDeckStats" action.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeckStatsParams {
    /// Names of the decks for which to retrieve statistics.
    pub decks: Vec<String>,
}

/// Statistics for a single deck.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDeckStats {
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

/// Gets statistics such as total cards and cards due for the given decks.
pub fn get_deck_stats(params: GetDeckStatsParams) -> Result<HashMap<String, GetDeckStats>> {
    anki_connect_send("getDeckStats", Some(params))
}
