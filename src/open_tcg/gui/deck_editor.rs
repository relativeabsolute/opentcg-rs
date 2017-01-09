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
use std::cell::RefCell;

use gtk::prelude::*;
use gtk::{Window, WindowPosition,
    Builder, Orientation, Frame, FlowBox, RadioButton, SelectionData};
use gtk::Box as GtkBox;

use self::gdk::{Screen, EventButton, DragContext};

use open_tcg::game::tcg::TCG;
use open_tcg::game::deck::Deck;
use open_tcg::game::card::CardInfo;
use super::card_display::CardDisplay;
use super::card_search::CardSearch;
use super::card_view::{CardView, CardViewType};
use super::image_manager::ImageManager;

struct DragInfo {
    source_type : CardViewType,
    source_data : String
}

pub struct DeckEditor {
    window : Window,
    editor_box : GtkBox,
    card_display : CardDisplay,
    card_search : Rc<CardSearch>,
    // TODO: add controls here
    current_tcg : Rc<TCG>,
    img_manager : Rc<ImageManager>,
    deck_view : Frame,
    section_views : Vec<Rc<CardView>>,
    section_buttons : Vec<RadioButton>,
    current_deck : Deck,
    drag_info : RefCell<Option<DragInfo>>
}

impl DeckEditor {
    pub fn new(tcg : Rc<TCG>) -> Rc<DeckEditor> {
        let instance = Rc::new(DeckEditor::init_controls(tcg));

        // TODO: load deck that was last being edited
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
        let tcg_clone = tcg.clone();
        let mut instance = DeckEditor{window : builder.get_object("window").unwrap(),
            card_display : CardDisplay::new(tcg.clone(), img_manager.clone()), 
            card_search : CardSearch::new(tcg.clone(), img_manager.clone()),
            current_tcg : tcg,
            editor_box : builder.get_object("editor_box").unwrap(),
            img_manager : img_manager,
            deck_view : Frame::new(Some("Deck")),
            section_views : Vec::new(),
            section_buttons : Vec::new(),
            current_deck : tcg_clone.new_deck(),
            drag_info : RefCell::new(None)};
        

        instance.init_deck_views();
        instance.editor_box.pack_start(&instance.card_display.frame, false, false, 0);
        instance.editor_box.pack_start(&instance.deck_view, true, true, 0);
        instance.editor_box.pack_end(&instance.card_search.frame, false, false, 0);
        instance.editor_box.reorder_child(&instance.card_display.frame, 0);


        instance
    }

    fn init_deck_views(&mut self) {

        let views_box = GtkBox::new(Orientation::Vertical, 10);

        let tcg_clone = self.current_tcg.clone();
        for section in tcg_clone.sections.iter() {
            let section_frame = Frame::new(Some(&section.name));

            let section_view = CardView::new_with_size(CardViewType::EditorView, self.current_tcg.clone(), self.img_manager.clone(),
                section.rows as usize, section.columns as usize);

            section_frame.add(&section_view.grid);

            views_box.pack_start(&section_frame, false, false, 0);
            self.section_views.push(section_view);
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

    fn on_card_view_drag_data_get(&self, view : &CardView, context : &DragContext, data : &SelectionData, info : u32, time : u32) {
        if let Some(text) = view.get_dragged_text() {
            *self.drag_info.borrow_mut() = Some(DragInfo{source_type : view.get_view_type(),
                source_data : text.clone()});
        }
    }

    fn on_deck_view_drag_drop(&self, index : usize, view : &CardView, context : &DragContext, x : i32, y : i32, time : u32) {
        if let Some(ref info) = *self.drag_info.borrow() {
            if let CardViewType::SearchView = info.source_type {
                if let Some(card_info) = self.current_tcg.cards.get(&info.source_data) {
                    self.add_card_to_section(index, view, card_info);
                }
            }
        }
    }

    fn add_card_to_section(&self, index : usize, view : &CardView, info : &CardInfo) {
        let mut section_cards = self.current_deck.sections[index].cards.borrow_mut();
        if section_cards.contains_key(&info.name) {
            let entry = section_cards.get_mut(&info.name);
            if let Some(copies) = entry {
                if *copies < self.current_tcg.card_limit {
                    *copies += 1;
                    view.add_card(&info);
                }
            }
        } else {
            section_cards.insert(info.name.clone(), 1);
            view.add_card(&info);
        }
    }

    fn on_card_search_drag_drop(&self, view : &CardView, context : &DragContext, x : i32, y : i32, time : u32) {
        if let Some(ref info) = *self.drag_info.borrow() {
            if let CardViewType::EditorView = info.source_type {
                if let Some(card_info) = self.current_tcg.cards.get(&info.source_data) {
                    // TODO: remove card 
                    println!("Card will be removed");
                }
            }
        }
    }

    fn connect_events(instance : Rc<DeckEditor>) {
        {
            let instance_copy = instance.clone();
            instance.card_search.connect_card_hover(move |_, name, _| {
                instance_copy.card_display.set_card(name);
            });
        }
        {
            let instance_copy = instance.clone();
            instance.card_search.connect_card_drag_data_get(move |view, context, data, info, time| {
                instance_copy.on_card_view_drag_data_get(view, context, data, info, time);
            });
        }
        {
            let instance_copy = instance.clone();
            instance.card_search.connect_view_drag_drop(move |view, context, x, y, time| {
                instance_copy.on_card_search_drag_drop(view, context, x, y, time); 
            });
        }
        /*
        {
            let instance_copy = instance.clone();
            // TODO: check type of the eventbutton to ensure only card can be added
            // maybe on double click?
            instance.card_search.connect_card_clicked(move |_, name, _| {
                instance_copy.add_card(name);
            });
        }
        */
        for i in 0..instance.section_views.len() {
            {
                let instance_copy = instance.clone();
                instance.section_views[i].connect_card_drag_data_get(move |view, context, data, info, time| {
                    instance_copy.on_card_view_drag_data_get(view, context, data, info, time);
                });
            }
            {
                let instance_copy = instance.clone();
                instance.section_views[i].connect_view_drag_drop(move |view, context, x, y, time| {
                    instance_copy.on_deck_view_drag_drop(i, view, context, x, y, time);
                });
            }
        }
    }

    fn on_section_view_clicked(&self, widget : &CardView, name : &String, evt : &EventButton) {
        if evt.as_ref().button == 3 {
            widget.remove_card(name);
        }
    }
}
