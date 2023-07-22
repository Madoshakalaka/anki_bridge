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

pub use crate::{
    card_actions::{
        are_due::*, are_suspended::*, cards_info::*, cards_mod_times::*, cards_to_notes::*,
        find_cards::*, forget_cards::*, get_ease_factors::*, get_intervals::*,
        get_intervals_alternative::*, relearn_cards::*, set_ease_factors::*,
        set_specific_value_of_card::*, suspend::*, suspended::*, unsuspend::*,
    },
    deck_actions::{
        change_deck::*, clone_deck_config_id::*, create_deck::*, deck_name_and_ids::*,
        deck_names::*, delete_decks::*, get_deck_config::*, get_deck_stats::*, get_decks::*,
        remove_deck_config_id::*, save_deck_config::*, set_deck_config_id::*,
    },
    graphical_actions::{
        gui_add_cards::*, gui_answer_card::*, gui_browse::*, gui_check_database::*,
        gui_current_card::*, gui_deck_browser::*, gui_deck_overview::*, gui_deck_review::*,
        gui_edit_note::*, gui_exit_anki::*, gui_selected_notes::*, gui_show_answer::*,
        gui_show_question::*, gui_start_card_timer::*,
    },
    miscellaneous_actions::{
        api_reflect::*, export_package::*, get_profiles::*, import_package::*, load_profile::*,
        multi::*, reload_collection::*, request_permission::*, sync::*, version::*,
    },
    statistic_actions::{
        card_reviews::*, get_collection_stats_html::*, get_latest_review_id::*,
        get_num_cards_reviewed_by_day::*, get_num_cards_reviewed_today::*, get_reviews_of_cards::*,
        insert_reviews::*,
    },
    AnkiClient, AnkiRequestable,
};
