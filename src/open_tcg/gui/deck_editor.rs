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
extern crate gdk;

use std::rc::Rc;
use gtk::prelude::*;
use gtk::{Window, WindowPosition,
    Builder, Orientation, Frame};
use gtk::Box as GtkBox;

use self::gdk::Screen;

use open_tcg::game::tcg::TCG;
use super::card_display::CardDisplay;
use super::card_search::CardSearch;
use super::card_view::CardView;
use super::image_manager::ImageManager;

pub struct DeckEditor {
    window : Window,
    editor_box : GtkBox,
    card_display : CardDisplay,
    card_search : Rc<CardSearch>,
    // TODO: add controls here
    current_tcg : Rc<TCG>,
    img_manager : Rc<ImageManager>,
    deck_view : Frame
}

impl DeckEditor {
    pub fn new(tcg : Rc<TCG>) -> Rc<DeckEditor> {
        let instance = Rc::new(DeckEditor::init_controls(tcg));


        // TODO: fill in control setup and event connections
        DeckEditor::connect_events(instance.clone());

        instance.determine_size();

        // TODO: set title based on current deck
        instance.window.set_title("DeckEdit");
        instance.window.show_all();
        instance
    }

    fn init_controls(tcg : Rc<TCG>) -> DeckEditor {
        let glade_src = include_str!("deck_editor.glade");
        let builder = Builder::new_from_string(glade_src);

        let img_manager = Rc::new(ImageManager::new());
        let instance = DeckEditor{window : builder.get_object("window").unwrap(),
            card_display : CardDisplay::new(tcg.clone(), img_manager.clone()), 
            card_search : CardSearch::new(tcg.clone(), img_manager.clone()),
            current_tcg : tcg,
            editor_box : builder.get_object("editor_box").unwrap(),
            img_manager : img_manager,
            deck_view : Frame::new(Some("Deck"))};
        

        instance.init_deck_views();
        instance.editor_box.pack_start(&instance.card_display.frame, false, false, 0);
        instance.editor_box.pack_start(&instance.deck_view, false, false, 0);
        instance.editor_box.pack_start(&instance.card_search.frame, false, false, 0);


        instance
    }

    fn init_deck_views(&self) {

        let views_box = GtkBox::new(Orientation::Vertical, 10);

        let tcg_clone = self.current_tcg.clone();
        for section in tcg_clone.sections.iter() {
            let section_frame = Frame::new(Some(&section.name));

            let section_view = CardView::new_with_size(self.current_tcg.clone(), self.img_manager.clone(),
                section.rows as usize, section.columns as usize);

            section_frame.add(&section_view.grid);

            views_box.pack_start(&section_frame, false, false, 0);
        }

        self.deck_view.add(&views_box);

    }

    fn determine_size(&self) {
        if let Some(scr) = Screen::get_default() {
            let width = scr.get_width() as f64;
            let height = scr.get_height() as f64;

            let new_width = width * 0.75;
            let new_height = height * 0.75;

            self.window.set_default_size(new_width as i32, new_height as i32);
            self.window.set_position(WindowPosition::Center);

            let deck_view_width = new_width * 0.5;
            let deck_view_height = new_height;
            self.deck_view.set_size_request(deck_view_width as i32, deck_view_height as i32);
        }
    }

    fn connect_events(instance : Rc<DeckEditor>) {
        {
            let instance_copy = instance.clone();
            instance.card_search.connect_card_hover(move |_, name, _| {
                instance_copy.card_display.set_card(name);
            });
        }
    }
}
