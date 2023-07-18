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

/// Moves cards with the given IDs to a different deck, creating the deck if it doesn’t exist yet.
pub mod change_deck;

/// Creates a new configuration group with the given name, cloning from the group with the given
/// ID, or from the default group if this is unspecified. Returns the ID of the new configuration
/// group, or [false] if the specified group to clone from does not exist.
pub mod clone_deck_config_id;

/// Create a new empty deck. Will not overwrite a deck that exists with the same name.
pub mod create_deck;

/// Gets the complete list of deck names for the current user.
pub mod deck_name_and_ids;

/// Gets the complete list of deck names for the current user.
pub mod deck_names;

/// Deletes decks with the given names. The argument [`cards_too`](delete_decks::DeleteDecksRequest::cards_too) must be specified and set to true.
pub mod delete_decks;

/// Gets the configuration group object for the given deck.
pub mod get_deck_config;

/// Gets statistics such as total cards and cards due for the given decks.
pub mod get_deck_stats;

/// Accepts an array of card IDs and returns an object with each deck name as a key, and its value an array of the given cards which belong to it.
pub mod get_decks;

/// Removes the configuration group with the given ID, returning [true] if successful, or [false] if attempting to remove either the default configuration group (ID = 1) or a configuration group that does not exist.
pub mod remove_deck_config_id;

/// Saves the given configuration group, returning [true] on success or [false] if the ID of the configuration group is invalid (such as when it does not exist).
pub mod save_deck_config;

/// Changes the configuration group for the given decks to the one with the given ID. Returns
/// [true] on success or [false] if the given configuration group or any of the given decks do not exist.
pub mod set_deck_config_id;
