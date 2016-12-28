// MIT License

// Copyright (c) 2016 Johan Burke

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

extern crate gtk;

use std::rc::Rc;

use gtk::prelude::*;
use gtk::{Builder, Button, Frame, Image, TextView, Label,
    SearchEntry, ComboBoxText};
use gtk::Box as GtkBox;
use open_tcg::game::tcg::TCG;
use super::card_view::CardView;

pub struct CardSearch {
    pub frame : Frame,
    card_name_search : SearchEntry,
    card_text_search : SearchEntry,
    type_choice : ComboBoxText,
    search_items_box : GtkBox,
    card_view : CardView,
    update_button : Button,
    clear_button : Button,
    current_tcg : Rc<TCG>
}

impl CardSearch {

    pub fn new(tcg : Rc<TCG>) -> Rc<CardSearch> {
        let instance = Rc::new(CardSearch::init_controls(tcg));
        
        CardSearch::connect_events(instance.clone());

        instance
    }

    fn init_controls(tcg : Rc<TCG>) -> CardSearch {
        let glade_src = include_str!("card_search.glade");
        let builder = Builder::new_from_string(glade_src);

        let mut instance = CardSearch{frame : builder.get_object("card_search").unwrap(),
            current_tcg : tcg,
            card_name_search : builder.get_object("card_name_search").unwrap(),
            card_text_search : builder.get_object("card_text_search").unwrap(),
            type_choice : builder.get_object("type_choice").unwrap(),
            search_items_box : builder.get_object("search_items_box").unwrap(),
            update_button : builder.get_object("update_button").unwrap(),
            clear_button : builder.get_object("clear_button").unwrap(),
            card_view : CardView::new()};
        
        instance.type_choice.append(None, "All Types");
        for type_name in instance.current_tcg.card_types.keys() {
            instance.type_choice.append(None, &type_name);
        }
        instance.type_choice.set_active(0);

        instance.search_items_box.pack_end(&instance.card_view.view, false, false, 0);

        instance
    }

    fn connect_events(instance : Rc<CardSearch>) {
        {
            let instance_copy = instance.clone();
            instance.update_button.connect_clicked(move |_| {
                instance_copy.on_update_clicked();
            });
        }
        {
            let instance_copy = instance.clone();
            instance.clear_button.connect_clicked(move |_| {
                instance_copy.on_clear_clicked();
            });
        }
    }

    fn on_update_clicked(&self) {
        // TODO: update grid of card_view with cards
        // meeting the current search criteria
    }

    fn on_clear_clicked(&self) {
        self.card_name_search.set_text("");
        self.card_text_search.set_text("");
        self.type_choice.set_active(0);
    }
}
