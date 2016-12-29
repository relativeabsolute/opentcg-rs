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
use std::cell::RefCell;
use gtk::prelude::*;
use gtk::{Window, WindowType, Builder, Button, Orientation};
use gtk::Box as GtkBox;

use open_tcg::game::tcg::TCG;
use super::card_display::CardDisplay;
use super::card_search::CardSearch;
use super::image_manager::ImageManager;

pub struct DeckEditor {
    window : Window,
    editor_box : GtkBox,
    card_display : CardDisplay,
    card_search : Rc<CardSearch>,
    // TODO: add controls here
    current_tcg : Rc<TCG>,
    img_manager : Rc<ImageManager>
}

impl DeckEditor {
    pub fn new(tcg : Rc<TCG>) -> Rc<DeckEditor> {
        let instance = Rc::new(DeckEditor::init_controls(tcg));


        // TODO: fill in control setup and event connections

        instance.window.show_all();
        instance
    }

    fn init_controls(tcg : Rc<TCG>) -> DeckEditor {
        let glade_src = include_str!("deck_editor.glade");
        let builder = Builder::new_from_string(glade_src);

        let img_manager = Rc::new(ImageManager::new());
        let mut instance = DeckEditor{window : builder.get_object("window").unwrap(),
            card_display : CardDisplay::new(tcg.clone()), 
            card_search : CardSearch::new(tcg.clone(), img_manager.clone()),
            current_tcg : tcg,
            editor_box : builder.get_object("editor_box").unwrap(),
            img_manager : img_manager};
        
        
        instance.editor_box.pack_start(&instance.card_display.frame, false, false, 0);
        instance.editor_box.pack_end(&instance.card_search.frame, false, false, 0);

        instance
    }
}
