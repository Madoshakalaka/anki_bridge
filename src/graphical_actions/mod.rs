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

/// Invokes the Add Cards dialog, presets the note using the given deck and model, with the provided field values and tags. Invoking it multiple times closes the old window and reopen the window with the new provided values.
/// Audio, video, and picture files can be embedded into the fields via the `audio`, `video`, and
/// `picture` keys, respectively. Refer to the documentation of `addNote` and `storeMediaFile` for an explanation of these fields.
/// The result is the ID of the note which would be added, if the user chose to confirm the Add Cards dialogue.
pub mod gui_add_cards;

/// Answers the current card; returns [true] if succeeded or [false] otherwise. Note that the answer for the current card must be displayed before before any answer can be accepted by Anki.
pub mod gui_answer_card;

/// Invokes the Card Browser dialog and searches for a given query. Returns an array of identifiers
/// of the cards that were found. Query syntax is [documented here](https://docs.ankiweb.net/searching.html).
pub mod gui_browse;

/// Requests a database check, but returns immediately without waiting for the check to complete.
/// Therefore, the action will always return [true] even if errors are detected during the database check.
pub mod gui_check_database;

/// Returns information about the current card or `null` if not in review mode.
pub mod gui_current_card;

pub mod gui_deck_browser;

/// Opens the Deck Overview dialog for the deck with the given name; returns [true] if succeeded or
/// [false] otherwise.
pub mod gui_deck_overview;

/// Starts review for the deck with the given name; returns [true] if succeeded or [false] otherwise.
pub mod gui_deck_review;

/// Opens the Edit dialog with a note corresponding to given note ID. The dialog is similar to the Edit Current dialog, but:
/// - has a Preview button to preview the cards for the note
/// - has a Browse button to open the browser with these cards
/// - has Previous/Back buttons to navigate the history of the dialog
/// - has no bar with the Close button
pub mod gui_edit_note;

/// Schedules a request to gracefully close Anki. This operation is asynchronous, so it will return immediately and won’t wait until the Anki process actually terminates.
pub mod gui_exit_anki;

/// Finds the open instance of the Card Browser dialog and returns an array of identifiers of the notes that are selected. Returns an empty list if the browser is not open.
pub mod gui_selected_notes;

/// Shows answer text for the current card; returns [true] if in review mode or [false] otherwise.
pub mod gui_show_answer;

/// Shows question text for the current card; returns [true] if in review mode or [false] otherwise.
pub mod gui_show_question;

/// Starts or resets the `timerStarted` value for the current card. This is useful for deferring
/// the start time to when it is displayed via the API, allowing the recorded time taken to answer
/// the card to be more accurate when calling `guiAnswerCard`.
pub mod gui_start_card_timer;
