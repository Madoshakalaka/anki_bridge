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

/// Returns an array indicating whether each of the given cards is due (in the same order). Note: cards in the learning queue with a large interval (over 20 minutes) are treated as not due until the time of their interval has passed, to match the way Anki treats them when reviewing.
pub mod are_due;

/// Returns an array indicating whether each of the given cards is suspended (in the same order).
/// If card doesn’t exist returns `null`.
pub mod are_suspended;

/// Returns a list of objects containing for each card ID the card fields, front and back sides including CSS, note type, the note that the card belongs to, and deck name, last modification timestamp as well as ease and interval.
pub mod cards_info;

/// Returns a list of objects containings for each card ID the modification time. This function is
/// about 15 times faster than executing `cardsInfo`.
pub mod cards_mod_times;

/// Returns an unordered array of note IDs for the given card IDs. For cards with the same note, the ID is only given once in the array.
pub mod cards_to_notes;

/// Returns an array of card IDs for a given query. Functionally identical to `guiBrowse` but doesn’t use the GUI for better performance.
pub mod find_cards;

/// Forget cards, making the cards new again.
pub mod forget_cards;

/// Returns an array with the ease factor for each of the given cards (in the same order).
pub mod get_ease_factors;

/// Returns an array of the most recent intervals for each given card ID, or a 2-dimensional array
/// of all the intervals for each given card ID when `complete` is [true]. Negative intervals are in seconds and positive intervals in days.
pub mod get_intervals;

/// Returns an array of the most recent intervals for each given card ID, or a 2-dimensional array
/// of all the intervals for each given card ID when `complete` is [true]. Negative intervals are in seconds and positive intervals in days.
pub mod get_intervals_alternative;

/// Make cards be “relearning”.
pub mod relearn_cards;

/// Sets ease factor of cards by card ID; returns [true] if successful (all cards existed) or [false] otherwise.
pub mod set_ease_factors;

/// Sets specific value of a single card. Given the risk of wreaking havor in the database when
/// changing some of the values of a card, some of the keys require the argument “`warning_check`”
/// set to True. This can be used to set a card’s flag, change it’s ease factor, change the review
/// order in a filtered deck and change the column “data” (not currently used by anki apparantly),
/// and many other values. A list of values and explanation of their respective utility can be
/// found at [AnkiDroid’s wiki](https://github.com/ankidroid/Anki-Android/wiki/Database-Structure).
pub mod set_specific_value_of_card;

/// Suspend cards by card ID; returns [true] if successful (at least one card wasn’t already
/// suspended) or [false] otherwise.
pub mod suspend;

/// Check if card is suspended by its ID. Returns [true] if suspended, [false] otherwise.
pub mod suspended;

/// Unsuspend cards by card ID; returns [true] if successful (at least one card was previously
/// suspended) or [false] otherwise.
pub mod unsuspend;
