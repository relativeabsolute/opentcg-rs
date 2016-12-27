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

use open_tcg::game::tcg::TCG;

pub struct CardSearch {
    pub frame : Frame,
    card_name_search : SearchEntry,
    card_text_search : SearchEntry,
    type_choice : ComboBoxText,
    current_tcg : Rc<TCG>
}

impl CardSearch {

    pub fn new(tcg : Rc<TCG>) -> Rc<CardSearch> {
        let instance = Rc::new(CardSearch::init_controls(tcg));

        instance
    }

    fn init_controls(tcg : Rc<TCG>) -> CardSearch {
        let glade_src = include_str!("card_search.glade");
        let builder = Builder::new_from_string(glade_src);

        let mut instance = CardSearch{frame : builder.get_object("card_search").unwrap(),
            current_tcg : tcg,
            card_name_search : builder.get_object("card_name_search").unwrap(),
            card_text_search : builder.get_object("card_text_search").unwrap(),
            type_choice : builder.get_object("type_choice").unwrap()};
        
        for type_name in instance.current_tcg.card_types.keys() {
            instance.type_choice.append(None, &type_name);
        }

        instance
    }
}
