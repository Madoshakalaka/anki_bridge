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

/// Parameters for the Card Browser dialog.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GuiBrowseParams {
    /// The search query for the Card Browser dialog.
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

/// Parameters for adding cards using the Add Cards dialog.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GuiAddCardsParams {
    /// The note to add using the Add Cards dialog.
    pub note: GuiAddCardsNote,
}

/// A note for adding cards using the Add Cards dialog.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuiAddCardsNoteFields {
    /// The "Text" field of the note.
    #[serde(rename = "Text")]
    pub text: String,
    /// The "Extra" field of the note.
    #[serde(rename = "Extra")]
    pub extra: String,
}

/// Picture attached to the note for adding cards using the Add Cards dialog.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuiAddCardsNotePicture {
    /// The URL of the picture.
    pub url: String,
    /// The filename of the picture.
    pub filename: String,
    /// The fields associated with the picture.
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

/// Parameters for editing a note using the Edit dialog.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GuiEditNoteParams {
    /// The ID of the note to edit.
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the current card being reviewed.
pub struct GuiCurrentCard {
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
    pub fields: GuiCurrentCardFields,
    /// The template of the current card.
    pub template: String,
    /// The ID of the current card.
    pub card_id: isize,
    /// The buttons associated with the current card.
    pub buttons: Vec<isize>,
    /// The next reviews for the current card.
    pub next_reviews: Vec<String>,
}

/// Represents the fields of the current card being reviewed.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuiCurrentCardFields {
    /// The front face of the fields.
    #[serde(rename = "Front")]
    /// The back face of the fields.
    pub front: GuiCurrentCardFieldsFace,
    #[serde(rename = "Back")]
    pub back: GuiCurrentCardFieldsFace,
}

/// Represents a face of the fields of the current card being reviewed.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuiCurrentCardFieldsFace {
    /// The value of the face.
    pub value: String,
    /// The order of the face.
    pub order: isize,
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

/// Parameters for answering the current card.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GuiAnswerCardParams {
    pub cards: String,
}

/// Answers the current card; returns [true] if succeeded or [false] otherwise. Note that the answer for the current card must be displayed before before any answer can be accepted by Anki.
#[maybe_async::maybe_async]
pub async fn gui_answer_card(params: GuiAnswerCardParams) -> Result<bool> {
    anki_connect_send("guiAnswerCard", Some(params)).await
}

/// Parameters for opening the Deck Overview dialog.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GuiDeckOverviewParams {
    /// The name of the deck.
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

/// Parameters for starting a deck review.
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GuiDeckReviewParams {
    /// The name of the deck.
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
