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

/// Parameters for the "getEaseFactors" action.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetEaseFactorsParams {
    /// IDs of the cards for which to retrieve ease factors.
    pub cards: Vec<usize>,
}

/// Returns an array with the ease factor for each of the given cards (in the same order).
pub fn get_ease_factors(params: GetEaseFactorsParams) -> Result<Vec<usize>> {
    anki_connect_send("getEaseFactors", Some(params))
}

/// Parameters for the "setEaseFactors" action.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetEaseFactorsParams {
    /// IDs of the cards for which to set ease factors.
    pub cards: Vec<usize>,
    /// Ease factors to set for the corresponding cards.
    pub ease_factors: Vec<usize>,
}

/// Sets ease factor of cards by card ID; returns [true] if successful (all cards existed) or [false] otherwise.
pub fn set_ease_factors(params: SetEaseFactorsParams) -> Result<Vec<bool>> {
    anki_connect_send("setEaseFactors", Some(params))
}

/// Parameters for the "setSpecificValueOfCard" action.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSpecificValueOfCardParams {
    /// ID of the card for which to set specific values.
    pub card: usize,
    /// Keys of the specific values to set.
    pub keys: Vec<String>,
    /// New values to set for the corresponding keys.
    pub new_values: Vec<String>,
}

/// Sets specific value of a single card. Given the risk of wreaking havor in the database when
/// changing some of the values of a card, some of the keys require the argument “warning_check”
/// set to True. This can be used to set a card’s flag, change it’s ease factor, change the review
/// order in a filtered deck and change the column “data” (not currently used by anki apparantly),
/// and many other values. A list of values and explanation of their respective utility can be
/// found at [AnkiDroid’s wiki](https://github.com/ankidroid/Anki-Android/wiki/Database-Structure).
pub fn set_specific_value_of_card(params: SetSpecificValueOfCardParams) -> Result<Vec<bool>> {
    anki_connect_send("setSpecificValueOfCard", Some(params))
}

/// Parameters for the "suspend" action.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SuspendParams {
    /// IDs of the cards to suspend.
    pub cards: Vec<usize>,
}

/// Suspend cards by card ID; returns [true] if successful (at least one card wasn’t already
/// suspended) or [false] otherwise.
pub fn suspend(params: SuspendParams) -> Result<bool> {
    anki_connect_send("suspend", Some(params))
}

/// Parameters for the "unsuspend" action.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UnsuspendParams {
    /// IDs of the cards to unsuspend.
    pub cards: Vec<usize>,
}

/// Unsuspend cards by card ID; returns [true] if successful (at least one card was previously
/// suspended) or [false] otherwise.
pub fn unsuspend(params: UnsuspendParams) -> Result<bool> {
    anki_connect_send("unsuspend", Some(params))
}

/// Parameters for the "suspended" action.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SuspendedParams {
    /// ID of the card to check.
    pub card: usize,
}

/// Check if card is suspended by its ID. Returns [true] if suspended, [false] otherwise.
pub fn suspended(params: SuspendedParams) -> Result<bool> {
    anki_connect_send("suspended", Some(params))
}

/// Parameters for the "areSuspended" action.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AreSuspendedParams {
    /// IDs of the cards to check.
    pub cards: Vec<usize>,
}

/// Returns an array indicating whether each of the given cards is suspended (in the same order).
/// If card doesn’t exist returns `null`.
pub fn are_suspended(params: AreSuspendedParams) -> Result<Vec<bool>> {
    anki_connect_send("areSuspended", Some(params))
}

/// Parameters for the "areDue" action.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AreDueParams {
    /// IDs of the cards to check.
    pub cards: Vec<usize>,
}

/// Returns an array indicating whether each of the given cards is due (in the same order). Note: cards in the learning queue with a large interval (over 20 minutes) are treated as not due until the time of their interval has passed, to match the way Anki treats them when reviewing.
pub fn are_due(params: AreDueParams) -> Result<Vec<bool>> {
    anki_connect_send("areDue", Some(params))
}

/// Parameters for the "getIntervals" action.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIntervalsParams {
    /// IDs of the cards to retrieve intervals for.
    pub cards: Vec<usize>,
}

/// Returns an array of the most recent intervals for each given card ID, or a 2-dimensional array
/// of all the intervals for each given card ID when `complete` is [true]. Negative intervals are in seconds and positive intervals in days.
pub fn get_intervals(params: GetIntervalsParams) -> Result<Vec<isize>> {
    anki_connect_send("getIntervals", Some(params))
}

/// Parameters for the "getIntervals" action alternative.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIntervalsAlternativeParams {
    /// IDs of the cards to retrieve intervals for.
    pub cards: Vec<usize>,
    /// Determines whether to return a complete 2-dimensional array of intervals.
    pub complete: bool,
}

/// Returns an array of the most recent intervals for each given card ID, or a 2-dimensional array
/// of all the intervals for each given card ID when `complete` is [true]. Negative intervals are in seconds and positive intervals in days.
pub fn get_intervals_alternative(params: GetIntervalsAlternativeParams) -> Result<Vec<Vec<isize>>> {
    anki_connect_send("getIntervals", Some(params))
}

/// Parameters for the "findCards" action.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct FindCardsParams {
    /// The query string used to search for cards.
    pub query: String,
}

/// Returns an array of card IDs for a given query. Functionally identical to `guiBrowse` but doesn’t use the GUI for better performance.
pub fn find_cards(params: FindCardsParams) -> Result<Vec<usize>> {
    anki_connect_send("findCards", Some(params))
}

/// Parameters for the "cardsToNotes" action.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CardsToNotesParams {
    /// The card IDs for which to retrieve the corresponding note IDs.
    pub cards: Vec<usize>,
}

/// Returns an unordered array of note IDs for the given card IDs. For cards with the same note, the ID is only given once in the array.
pub fn cards_to_notes(params: CardsToNotesParams) -> Result<Vec<usize>> {
    anki_connect_send("cardsToNotes", Some(params))
}

/// Parameters for the "cardsModTime" action.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CardsModTimeParams {
    /// The card IDs for which to retrieve the modification time.
    pub cards: Vec<usize>,
}

/// Represents the modification time of a card.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardsModTime {
    /// The ID of the card.
    pub card_id: usize,
    /// The modification time of the card.
    #[serde(rename = "mod")]
    pub mod_: usize,
}

/// Returns a list of objects containings for each card ID the modification time. This function is
/// about 15 times faster than executing `cardsInfo`.
pub fn cards_mod_times(params: CardsModTimeParams) -> Result<Vec<CardsModTime>> {
    anki_connect_send("cardsModTime", Some(params))
}

/// Parameters for retrieving information about cards.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CardsInfoParams {
    /// The list of card IDs.
    pub cards: Vec<usize>,
}

/// Represents the information about a card.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardsInfo {
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

/// Returns a list of objects containing for each card ID the card fields, front and back sides including CSS, note type, the note that the card belongs to, and deck name, last modification timestamp as well as ease and interval.
pub fn cards_info(params: CardsInfoParams) -> Result<Vec<CardsInfo>> {
    anki_connect_send("cardsInfo", Some(params))
}

/// Parameters for forgetting cards.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ForgetCardsParams {
    /// The list of card IDs to forget.
    pub cards: Vec<usize>,
}

/// Forget cards, making the cards new again.
pub fn forget_cards(params: ForgetCardsParams) -> Result<()> {
    anki_connect_send("forgetCards", Some(params))
}

/// Parameters for relearning cards.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RelearnCardsParams {
    /// The list of card IDs to relearn.
    pub cards: Vec<usize>,
}

/// Make cards be “relearning”.
pub fn relearn_cards(params: RelearnCardsParams) -> Result<()> {
    anki_connect_send("relearnCards", Some(params))
}
