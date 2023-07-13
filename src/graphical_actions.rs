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

use crate::{anki_connect_send, Result};

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GuiBrowseParams {
    pub query: String,
}

/// Invokes the Card Browser dialog and searches for a given query. Returns an array of identifiers
/// of the cards that were found. Query syntax is [documented here](https://docs.ankiweb.net/searching.html).
#[maybe_async::maybe_async]
pub async fn gui_browse(params: GuiBrowseParams) -> Result<Vec<usize>> {
    anki_connect_send("guiBrowse", Some(params)).await
}

/// Finds the open instance of the Card Browser dialog and returns an array of identifiers of the notes that are selected. Returns an empty list if the browser is not open.
#[maybe_async::maybe_async]
pub async fn gui_selected_notes() -> Result<Vec<usize>> {
    anki_connect_send::<(), _>("guiSelectedNotes", None).await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GuiAddCardsParams {
    pub note: GuiAddCardsNote,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuiAddCardsNote {
    pub deck_name: String,
    pub model_name: String,
    pub fields: GuiAddCardsNoteFields,
    pub tags: Vec<String>,
    pub picture: Vec<GuiAddCardsNotePicture>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuiAddCardsNoteFields {
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "Extra")]
    pub extra: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuiAddCardsNotePicture {
    pub url: String,
    pub filename: String,
    pub fields: Vec<String>,
}

/// Invokes the Add Cards dialog, presets the note using the given deck and model, with the provided field values and tags. Invoking it multiple times closes the old window and reopen the window with the new provided values.
/// Audio, video, and picture files can be embedded into the fields via the `audio`, `video`, and
/// `picture` keys, respectively. Refer to the documentation of `addNote` and `storeMediaFile` for an explanation of these fields.
/// The result is the ID of the note which would be added, if the user chose to confirm the Add Cards dialogue.
#[maybe_async::maybe_async]
pub async fn gui_add_cards(params: GuiAddCardsParams) -> Result<usize> {
    anki_connect_send("guiAddCards", Some(params)).await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GuiEditNoteParams {
    pub note: usize,
}

/// Opens the Edit dialog with a note corresponding to given note ID. The dialog is similar to the Edit Current dialog, but:
/// - has a Preview button to preview the cards for the note
/// - has a Browse button to open the browser with these cards
/// - has Previous/Back buttons to navigate the history of the dialog
/// - has no bar with the Close button
#[maybe_async::maybe_async]
pub async fn gui_edit_note(params: GuiEditNoteParams) -> Result<()> {
    anki_connect_send("guiEditNote", Some(params)).await
}

// TODO: Not implemented yet
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuiCurrentCard {
    pub answer: String,
    pub question: String,
    pub deck_name: String,
    pub model_name: String,
    pub field_order: isize,
    pub fields: GuiCurrentCardFields,
    pub template: String,
    pub card_id: isize,
    pub buttons: Vec<isize>,
    pub next_reviews: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuiCurrentCardFields {
    #[serde(rename = "Front")]
    pub front: GuiCurrentCardFieldsFace,
    #[serde(rename = "Back")]
    pub back: GuiCurrentCardFieldsFace,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuiCurrentCardFieldsFace {
    pub value: String,
    pub order: i64,
}

/// Returns information about the current card or `null` if not in review mode.
#[maybe_async::maybe_async]
pub async fn gui_current_card() -> Result<GuiCurrentCard> {
    anki_connect_send::<(), _>("guiCurrentCard", None).await
}

/// Starts or resets the `timerStarted` value for the current card. This is useful for deferring
/// the start time to when it is displayed via the API, allowing the recorded time taken to answer
/// the card to be more accurate when calling `guiAnswerCard`.
#[maybe_async::maybe_async]
pub async fn gui_start_card_timer() -> Result<()> {
    anki_connect_send::<(), _>("guiStartCardTimer", None).await
}

/// Shows question text for the current card; returns [true] if in review mode or [false] otherwise.
#[maybe_async::maybe_async]
pub async fn gui_show_question() -> Result<()> {
    anki_connect_send::<(), _>("guiShowQuestion", None).await
}

/// Shows answer text for the current card; returns [true] if in review mode or [false] otherwise.
#[maybe_async::maybe_async]
pub async fn gui_show_answer() -> Result<()> {
    anki_connect_send::<(), _>("guiShowAnswer", None).await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GuiAnswerCardParams {
    pub cards: String,
}

/// Answers the current card; returns [true] if succeeded or [false] otherwise. Note that the answer for the current card must be displayed before before any answer can be accepted by Anki.
#[maybe_async::maybe_async]
pub async fn gui_answer_card(params: GuiAnswerCardParams) -> Result<bool> {
    anki_connect_send("guiAnswerCard", Some(params)).await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GuiDeckOverviewParams {
    pub name: String,
}

/// Opens the Deck Overview dialog for the deck with the given name; returns [true] if succeeded or
/// [false] otherwise.
#[maybe_async::maybe_async]
pub async fn gui_deck_overview(params: GuiDeckOverviewParams) -> Result<bool> {
    anki_connect_send("guiDeckOverview", Some(params)).await
}

/// Opens the Deck Browser dialog.
#[maybe_async::maybe_async]
pub async fn gui_deck_browser() -> Result<()> {
    anki_connect_send::<(), _>("guiDeckBrowser", None).await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GuiDeckReviewParams {
    pub name: String,
}

/// Starts review for the deck with the given name; returns [true] if succeeded or [false] otherwise.
#[maybe_async::maybe_async]
pub async fn gui_deck_review(params: GuiDeckReviewParams) -> Result<bool> {
    anki_connect_send("guiDeckReview", Some(params)).await
}

/// Schedules a request to gracefully close Anki. This operation is asynchronous, so it will return immediately and won’t wait until the Anki process actually terminates.
#[maybe_async::maybe_async]
pub async fn gui_exit_anki() -> Result<()> {
    anki_connect_send::<(), _>("guiExitAnki", None).await
}

/// Requests a database check, but returns immediately without waiting for the check to complete.
/// Therefore, the action will always return [true] even if errors are detected during the database check.
#[maybe_async::maybe_async]
pub async fn gui_check_database() -> Result<bool> {
    anki_connect_send::<(), _>("guiCheckDatabase", None).await
}
