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

/// Requests all card reviews for a specified deck after a certain time. `startID` is the latest unix time not included in the result. Returns a list of 9-tuples (reviewTime, `cardID`, usn, buttonPressed, newInterval, previousInterval, newFactor, reviewDuration, reviewType)
pub mod card_reviews;

/// Gets the collection statistics report
pub mod get_collection_stats_html;

/// Returns the unix time of the latest review for the given deck. 0 if no review has ever been made for the deck.
pub mod get_latest_review_id;

/// Gets the number of cards reviewed as a list of pairs of (dateString, number)
pub mod get_num_cards_reviewed_by_day;

/// Gets the count of cards that have been reviewed in the current day (with day start time as configured by user in anki)
pub mod get_num_cards_reviewed_today;

/// Requests all card reviews for each card ID. Returns a dictionary mapping each card ID to a list of dictionaries of the format:
/// ```json
/// {
///     "id": reviewTime,
///     "usn": usn,
///     "ease": buttonPressed,
///     "ivl": newInterval,
///     "lastIvl": previousInterval,
///     "factor": newFactor,
///     "time": reviewDuration,
///     "type": reviewType,
/// }
/// ```
/// The reason why these key values are used instead of the more descriptive counterparts is because these are the exact key values used in Anki’s database.
pub mod get_reviews_of_cards;

/// Inserts the given reviews into the database. Required format: list of 9-tuples (reviewTime, `cardID`, usn, buttonPressed, newInterval, previousInterval, newFactor, reviewDuration, reviewType)
pub mod insert_reviews;
