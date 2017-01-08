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
use gtk::{Builder, Button, Frame, 
    SearchEntry, ComboBoxText, SelectionData};
use self::gdk::{EventButton, EventMotion, DragContext};
use gtk::Box as GtkBox;
use open_tcg::game::tcg::TCG;
use open_tcg::game::card::CardInfo;
use super::card_view::{CardView, CardViewType};
use super::image_manager::ImageManager;

pub struct CardSearch {
    pub frame : Frame,
    card_name_search : SearchEntry,
    card_text_search : SearchEntry,
    type_choice : ComboBoxText,
    search_items_box : GtkBox,
    card_view : Rc<CardView>, 
    update_button : Button,
    clear_button : Button,
    current_tcg : Rc<TCG>,
    card_clicked_events : RefCell<Vec<Box<Fn(&CardSearch, &String, &EventButton)>>>,
    card_hover_events : RefCell<Vec<Box<Fn(&CardSearch, &String, &EventMotion)>>>,
    card_drag_data_get_events : RefCell<Vec<Box<Fn(&CardView, &DragContext, &SelectionData, u32, u32)>>>,
    view_drag_drop_events : RefCell<Vec<Box<Fn(&CardView, &DragContext, i32, i32, u32)>>>
}

impl CardSearch {

    pub fn new(tcg : Rc<TCG>, img_manager : Rc<ImageManager>) -> Rc<CardSearch> {
        let instance = Rc::new(CardSearch::init_controls(tcg, img_manager));
        
        CardSearch::connect_events(instance.clone());

        instance
    }

    fn init_controls(tcg : Rc<TCG>, img_manager : Rc<ImageManager>) -> CardSearch {
        let glade_src = include_str!("card_search.glade");
        let builder = Builder::new_from_string(glade_src);

        let tcg_clone = tcg.clone();
        let instance = CardSearch{frame : builder.get_object("card_search").unwrap(),
            current_tcg : tcg,
            card_name_search : builder.get_object("card_name_search").unwrap(),
            card_text_search : builder.get_object("card_text_search").unwrap(),
            type_choice : builder.get_object("type_choice").unwrap(),
            search_items_box : builder.get_object("search_items_box").unwrap(),
            update_button : builder.get_object("update_button").unwrap(),
            clear_button : builder.get_object("clear_button").unwrap(),
            card_view : CardView::new(CardViewType::SearchView, tcg_clone, img_manager),
            card_clicked_events : RefCell::new(Vec::new()),
            card_hover_events : RefCell::new(Vec::new()),
            card_drag_data_get_events : RefCell::new(Vec::new()),
            view_drag_drop_events : RefCell::new(Vec::new())};
        
        instance.type_choice.append(None, "All Types");
        for type_name in instance.current_tcg.card_types.keys() {
            instance.type_choice.append(None, &type_name);
        }
        instance.type_choice.set_active(0);

        // TODO: add spacing
        instance.search_items_box.pack_start(&instance.card_view.grid, false, false, 0);

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
        // propogate card clicked and hover events upward to the deck editor
        {
            let instance_copy = instance.clone();
            instance.card_view.connect_card_clicked(move |_, name, evt| {
                instance_copy.fire_card_clicked(name, evt);
            });
        }
        {
            let instance_copy = instance.clone();
            instance.card_view.connect_card_hover(move |_, name, evt| {
                instance_copy.fire_card_hover(name, evt);
            });
        }
        {
            let instance_copy = instance.clone();
            instance.card_view.connect_card_drag_data_get(move |view, context, data, info, time| {
                instance_copy.fire_card_drag_data_get(view, context, data, info, time);
            });
        }
        {
            let instance_copy = instance.clone();
            instance.card_view.connect_view_drag_drop(move |view, context, x, y, time| {
                instance_copy.fire_view_drag_drop(view, context, x, y, time);
            });
        }
    }

    pub fn connect_card_clicked<F : Fn(&Self, &String, &EventButton) + 'static>(&self, f : F) {
        self.card_clicked_events.borrow_mut().push(Box::new(f));
    }

    pub fn connect_card_hover<F : Fn(&Self, &String, &EventMotion) + 'static>(&self, f : F) {
        self.card_hover_events.borrow_mut().push(Box::new(f));
    }

    pub fn connect_card_drag_data_get<F : Fn(&CardView, &DragContext, &SelectionData, u32, u32) + 'static>(&self, f : F) {
        self.card_drag_data_get_events.borrow_mut().push(Box::new(f));
    }

    pub fn connect_view_drag_drop<F : Fn(&CardView, &DragContext, i32, i32, u32) +'static>(&self, f : F) {
        self.view_drag_drop_events.borrow_mut().push(Box::new(f));
    }


    fn fire_card_clicked(&self, name : &String, evt : &EventButton) {
        for f in self.card_clicked_events.borrow().iter() {
            f(self, name, evt);
        }
    }

    fn fire_card_hover(&self, name : &String, evt : &EventMotion) {
        for f in self.card_hover_events.borrow().iter() {
            f(self, name, evt);
        }
    }

    fn fire_card_drag_data_get(&self, view : &CardView, context : &DragContext, data : &SelectionData, info : u32, time : u32) {
        for f in self.card_drag_data_get_events.borrow().iter() {
            f(view, context, data, info, time);
        }
    }

    fn fire_view_drag_drop(&self, view : &CardView, context : &DragContext, x : i32, y : i32, time : u32) {
        for f in self.view_drag_drop_events.borrow().iter() {
            f(view, context, x, y, time);
        }
    }


    fn on_update_clicked(&self) {
        // TODO: update grid of card_view with cards
        // meeting the current search criteria
        let mut cards : Vec<CardInfo> = self.current_tcg.cards.values().map(|c| c.clone()).collect();
        if let Some(text) = self.card_name_search.get_text() {
            cards = cards.iter().filter(|&c| c.name.contains(&text)).map(|c| c.clone()).collect();
        }
        if let Some(text) = self.card_text_search.get_text() {
            cards = cards.iter().filter(|&c| c.text.contains(&text)).map(|c| c.clone()).collect();
        }
        self.card_view.set_cards(&cards);
        // TODO: pass cards to cardview
    }

    fn on_clear_clicked(&self) {
        // TODO: not sure if the previous search results should be cleared as well
        self.card_name_search.set_text("");
        self.card_text_search.set_text("");
        self.type_choice.set_active(0);
    }
}
