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
extern crate gdk_sys as gdk_ffi;
extern crate gdk_pixbuf;
extern crate gtk_sys as gtk_ffi;
extern crate glib;

use std::rc::Rc;
use std::cell::RefCell;
use std::ptr;

use gtk::prelude::*;
use gtk::{Grid, Image, EventBox, SelectionData, Menu, MenuItem};
use self::gdk::{EventButton, EventMotion, DragContext};

use open_tcg::game::card::CardInfo;
use open_tcg::game::tcg::TCG;
use super::image_manager::ImageManager;

use self::glib::translate::*;

const DEFAULT_ROW_COUNT : usize = 5;
const DEFAULT_COL_COUNT : usize = 4;
const RIGHT_MOUSE_BUTTON : u32 = 3;

#[derive(Clone)]
pub enum CardViewType {
    SearchView,
    EditorView
}

pub struct CardView {
    pub grid : Grid,
    images : Vec<Image>,
    boxes : Vec<EventBox>,
    cards : RefCell<Vec<CardInfo>>,
    card_clicked_events : RefCell<Vec<Box<Fn(&CardView, &String, &EventButton)>>>,
    card_hover_events : RefCell<Vec<Box<Fn(&CardView, &String, &EventMotion)>>>,
    card_drag_data_get_events : RefCell<Vec<Box<Fn(&CardView, &DragContext, &SelectionData, u32, u32)>>>,
    card_drag_data_received_events : RefCell<Vec<Box<Fn(&CardView, &DragContext, i32, i32, &SelectionData, u32, u32)>>>,
    view_drag_drop_events : RefCell<Vec<Box<Fn(&CardView, &DragContext, i32, i32, u32)>>>,
    img_manager : Rc<ImageManager>,
    current_tcg : Rc<TCG>,
    row_count : usize,
    col_count : usize,
    targets : Vec<gtk_ffi::GtkTargetEntry>,
    view_type : CardViewType,
    dragged_text : RefCell<Option<String>>
}

impl CardView {
    pub fn new(view_type : CardViewType, tcg : Rc<TCG>, img_manager : Rc<ImageManager>) -> Rc<CardView> {
        CardView::new_with_size(view_type, tcg, img_manager, DEFAULT_ROW_COUNT, DEFAULT_COL_COUNT)
    }

    pub fn new_with_size(view_type : CardViewType, tcg : Rc<TCG>, img_manager : Rc<ImageManager>, row_count : usize, col_count : usize) -> Rc<CardView> {
        let instance = Rc::new(CardView::init_controls(view_type, tcg, img_manager, row_count, col_count));

        CardView::connect_events(instance.clone());

        instance

    }

    pub fn get_view_type(&self) -> CardViewType {
        self.view_type.clone()
    }

    fn connect_events(instance : Rc<CardView>) {
        for i in 0..instance.row_count {
            for j in 0..instance.col_count {
                let index = i * instance.col_count + j;
                {
                    let instance_copy = instance.clone();
                    instance.boxes[index].connect_motion_notify_event(move |_, evt| {
                        instance_copy.on_image_hover(index, evt);
                        Inhibit(true)
                    });
                }
                {
                    let instance_copy = instance.clone();
                    instance.boxes[index].connect_button_press_event(move |_, evt| {
                        if let Some(text) = instance_copy.boxes[index].get_tooltip_text() {
                            /*
                            let button = evt.as_ref().button as u32;
                            if button == RIGHT_MOUSE_BUTTON {
                                let menu = Menu::new();
                                for i in 0..instance_copy.current_tcg.sections.len() {
                                    let label = "Add to ".to_string() + &instance_copy.current_tcg.sections[i].name;
                                    let item = MenuItem::new_with_label(&label);
                                    let instance_copy2 = instance_copy.clone();
                                    item.connect_activate(move |_| {
                                        instance_copy2.on_add_menu_item_clicked(i);
                                    });
                                    menu.attach(&item, 0, 1, i as u32, (i + 1) as u32);
                                }
                                let mut (mx, my) = evt.get_position();
                                // TODO: this doesn't work...
                                menu.popup(None, None, move |_, x, y| {
                                    let mut tmp_x : &mut i32 = &(mx as i32);
                                    let mut tmp_y : &mut i32 = my as i32;
                                    x = tmp_x;
                                    y = tmp_y;
                                    true
                                }, button, 0);
                            }
                            */
             
                            instance_copy.on_image_click(&text, index, evt);
                        }
                        Inhibit(true)
                    });
                }
                // TODO: since there seem to be issues with drag and drop, temporary solution is to
                // add a group of radio buttons for each subsection
                {
                    let instance_copy = instance.clone();
                    instance.boxes[index].connect_drag_data_get(move |_, context, data, info, time| {
                        instance_copy.on_image_drag_data_get(index, context, data, info, time);
                    });
                }
                {
                    let instance_copy = instance.clone();
                    instance.boxes[index].connect_drag_begin(move |_, context| {
                        instance_copy.on_image_drag_begin(index, context);
                    });
                }
                {
                    let instance_copy = instance.clone();
                    instance.boxes[index].connect_drag_data_received(move |_, context, x, y, data, info, time| {
                        instance_copy.on_image_drag_data_received(index, context, x, y, data, info, time);
                    });
                }
                
                {
                    let instance_copy = instance.clone();
                    instance.boxes[index].connect_drag_drop(move |_, context, x, y, time| -> bool {
                        instance_copy.on_image_drag_drop(index, context, x, y, time)
                    });
                }
            }
        }
    }
    pub fn get_row_count(&self) -> usize {
        self.row_count
    }

    pub fn get_col_count(&self) -> usize {
        self.col_count
    }

    // TODO: change to drag and drop
    pub fn connect_card_clicked<F : Fn(&Self, &String, &EventButton) + 'static>(&self, f : F) {
        // we should still keep this even if we use drag and drop
        self.card_clicked_events.borrow_mut().push(Box::new(f));
    }

    pub fn connect_card_hover<F : Fn(&Self, &String, &EventMotion) + 'static>(&self, f : F) {
        self.card_hover_events.borrow_mut().push(Box::new(f));
    }

    pub fn connect_card_drag_data_get<F : Fn(&Self, &DragContext, &SelectionData, u32, u32) + 'static>(&self, f : F) {
        self.card_drag_data_get_events.borrow_mut().push(Box::new(f));
    }

    pub fn connect_card_drag_data_received<F : Fn(&Self, &DragContext, i32, i32, &SelectionData, u32, u32) + 'static>(&self, f : F) {
        self.card_drag_data_received_events.borrow_mut().push(Box::new(f));
    }

    pub fn connect_view_drag_drop<F : Fn(&Self, &DragContext, i32, i32, u32) +'static>(&self, f : F) {
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

    fn fire_card_drag_data_get(&self, context : &DragContext, data : &SelectionData, info : u32, time : u32) {
        for f in self.card_drag_data_get_events.borrow().iter() {
            f(self, context, data, info, time);
        }
    }

    fn fire_card_drag_data_received(&self, context : &DragContext, x : i32, y : i32, data : &SelectionData, info : u32, time : u32) {
        for f in self.card_drag_data_received_events.borrow().iter() {
            f(self, context, x, y, data, info, time);
        }
    }

    fn fire_view_drag_drop(&self, context : &DragContext, x : i32, y : i32, time : u32) {
        for f in self.view_drag_drop_events.borrow().iter() {
            f(self, context, x, y, time);
        }
    }
    
    fn on_image_drag_drop(&self, index : usize, context : &DragContext, x : i32, y : i32, time : u32) -> bool {
        println!("Drag dropped.");
        self.fire_view_drag_drop(context, x, y, time);
        true
    }

    fn on_image_drag_data_received(&self, index : usize, context : &DragContext, x : i32, y : i32, data : &SelectionData, info : u32, time : u32) {
        // here we accept regardless if the image is empty
        println!("Drag data received!");
        self.fire_card_drag_data_received(context, x, y, data, info, time);
    }

    // TODO: set up drag events conditionally
    fn on_image_drag_begin(&self, index : usize, context : &DragContext) {
        if let None = self.boxes[index].get_tooltip_text() {
            context.drag_abort(0);
        }
    }

    pub fn get_dragged_text(&self) -> Option<String> {
        match *self.dragged_text.borrow() {
            Some(ref text) => Some(text.clone()),
            None => None
        }
    }

    fn on_image_drag_data_get(&self, index : usize, context : &DragContext, data : &SelectionData, info : u32, time : u32) {
        println!("Drag data get called");
        if let Some(text) = self.boxes[index].get_tooltip_text() {
            println!("Text is {}", &text);
            // added seems to be false with the following, so set a field in the card view with the
            // text instead
            /*
            let mut new_data = data.clone();
            let added = new_data.set_text(&text, -1);
            println!("added = {}", added);
            if let Some(data_text) = new_data.get_text() {
                println!("Data is {}", data_text);
            }
            */
            *self.dragged_text.borrow_mut() = Some(text.clone());
            self.fire_card_drag_data_get(context, &data, info, time);
        } else {
            context.drag_abort(time);
        }
    }

    fn on_image_hover(&self, index : usize, evt : &EventMotion) {
        if let Some(text) = self.boxes[index].get_tooltip_text() {
            self.fire_card_hover(&text, evt);
        }
    }

    fn on_add_menu_item_clicked(&self, index : usize) {

    }

    // due to lifetime requirements, we can't handle popups here
    // therefore the tooltip will be checked in the closure
    // and this will only be called to handle any custom events
    fn on_image_click(&self, text : &String, index : usize, evt : &EventButton) {
        self.fire_card_clicked(text ,evt);
    }

    /// Set up the controls of the CardView.
    fn init_controls(view_type : CardViewType, tcg : Rc<TCG>, img_manager : Rc<ImageManager>, row_count : usize, col_count : usize) -> CardView {
        // the easiest way to do this seems to be to create an array of images
        // whose tooltips are the names of their corresponding cards
        // then we can simply set those lying past a certain index
        // to not be visible
        let count = row_count * col_count;
        let mut result = CardView{grid : Grid::new(),
            images : Vec::with_capacity(count),
            boxes : Vec::with_capacity(count),
            cards : RefCell::new(Vec::new()),
            img_manager : img_manager,
            current_tcg : tcg,
            card_clicked_events : RefCell::new(Vec::new()),
            card_hover_events : RefCell::new(Vec::new()),
            card_drag_data_get_events : RefCell::new(Vec::new()),
            card_drag_data_received_events : RefCell::new(Vec::new()),
            view_drag_drop_events : RefCell::new(Vec::new()),
            row_count : row_count,
            col_count : col_count,
            targets : Vec::new(),
            view_type : view_type,
            dragged_text : RefCell::new(None)};
        
        for i in 0..result.row_count {
            for j in 0..result.col_count {
                let index = i * result.col_count + j;
                let img = Image::new();
                result.images.push(img);
                let evt_box = EventBox::new();
                evt_box.set_above_child(false);
                evt_box.add(&result.images[index]);

                let text_entry = gtk_ffi::GtkTargetEntry{target : "STRING".to_glib_none().0, flags : (gtk_ffi::GTK_TARGET_SAME_APP | gtk_ffi::GTK_TARGET_OTHER_WIDGET).bits(), info : 0};
                result.targets.push(text_entry);

                unsafe {
                    let targets_ptr = result.targets.as_mut_ptr();
                    gtk_ffi::gtk_drag_source_set(evt_box.to_glib_none().0, gdk_ffi::GDK_BUTTON1_MASK, targets_ptr, 1, gdk_ffi::GDK_ACTION_COPY);
                    gtk_ffi::gtk_drag_dest_set(evt_box.to_glib_none().0, gtk_ffi::GTK_DEST_DEFAULT_ALL, targets_ptr, 1, gdk_ffi::GDK_ACTION_COPY);
                }
                // TODO: set up target entries
                //evt_box.drag_source_add_text_targets();
                result.boxes.push(evt_box);
                result.grid.attach(&result.boxes[index], j as i32, i as i32, 1, 1);
            }
        }

        result.img_manager.load_image(&"proxy".to_string());
        let empty_vec : Vec<CardInfo> = Vec::new();
        result.set_cards(&empty_vec);

        result

    }

    pub fn add_card(&self, card : &CardInfo) {
        self.cards.borrow_mut().push(card.clone());
        self.update_cards();
    }

    fn update_cards(&self) {
        // using CardInfos directly removes the need to keep an Rc to the current TCG
        let cards = self.cards.borrow();
        let cutoff = cards.len();

        let mut targets : Vec<gtk_ffi::GtkTargetEntry> = Vec::new();
        

        for i in 0..self.row_count {
            for j in 0..self.col_count {
                let index = i * self.col_count + j;
                if index < cutoff {
                    // TODO: unload previously loaded images
                    self.img_manager.load_image(&cards[index].set_code);
                    if let Some(img) = self.img_manager.get_small_image(&cards[index].set_code) {
                        self.images[index].set_from_pixbuf(Some(&img));
                        self.boxes[index].set_tooltip_text(Some(&cards[index].name));
                        
                        self.boxes[index].drag_source_set_icon_pixbuf(&img);
                    }
                } else {
                    if let Some(img) = self.img_manager.get_small_image(&"proxy".to_string()) {
                        self.images[index].set_from_pixbuf(Some(&img));
                        self.boxes[index].set_tooltip_text(None);
                    }
                }
            }
        }

    }

    /// Set the cards displayed by this CardView
    ///
    /// Updates the grid to the images corresponding to the given cards, and
    /// sets the remaining spaces to display a blank image.
    pub fn set_cards(&self, cards : &Vec<CardInfo>) {
        *self.cards.borrow_mut() = cards.clone();
        self.update_cards();
    }

    pub fn remove_card(&self, name : &String) {
        {
            let mut cards = self.cards.borrow_mut();
            if let Some(index) = cards.iter().position(|ref card| card.name == *name) {
                cards.remove(index);
            }
        }
        self.update_cards();
    }

    // TODO: display a single proxy card initially
    // TODO: connect mouse events
    // TODO: update images based upon collection of card names
    // TODO: figure out the best way to have the minimal amount of images loaded at once
    // TODO: create way to handle having more than 20 results
}

