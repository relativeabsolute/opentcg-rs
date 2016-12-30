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
extern crate gdk_pixbuf;

use std::rc::Rc;
use std::cell::RefCell;

use gtk::prelude::*;
use gtk::{Grid, Image, EventBox};
use self::gdk::{EventButton, EventMotion};

use open_tcg::game::card::CardInfo;
use super::image_manager::ImageManager;

const DEFAULT_ROW_COUNT : usize = 5;
const DEFAULT_COL_COUNT : usize = 4;

pub struct CardView {
    pub grid : Grid,
    images : Vec<Image>,
    boxes : Vec<EventBox>,
    card_clicked_events : RefCell<Vec<Box<Fn(&CardView, &String, &EventButton)>>>,
    card_hover_events : RefCell<Vec<Box<Fn(&CardView, &String, &EventMotion)>>>,
    img_manager : Rc<ImageManager>,
    row_count : usize,
    col_count : usize
}

impl CardView {
    pub fn new(img_manager : Rc<ImageManager>) -> Rc<CardView> {
        CardView::new_with_size(img_manager, DEFAULT_ROW_COUNT, DEFAULT_COL_COUNT)
    }

    pub fn new_with_size(img_manager : Rc<ImageManager>, row_count : usize, col_count : usize) -> Rc<CardView> {
        let instance = Rc::new(CardView::init_controls(img_manager, row_count, col_count));

        CardView::connect_events(instance.clone());

        instance

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
                        instance_copy.on_image_click(index, evt);
                        Inhibit(true)
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

    pub fn connect_card_clicked<F : Fn(&Self, &String, &EventButton) + 'static>(&self, f : F) {
        self.card_clicked_events.borrow_mut().push(Box::new(f));
    }

    pub fn connect_card_hover<F : Fn(&Self, &String, &EventMotion) + 'static>(&self, f : F) {
        self.card_hover_events.borrow_mut().push(Box::new(f));
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

    fn on_image_hover(&self, index : usize, evt : &EventMotion) {
        if self.images[index].get_visible() {
            self.fire_card_hover(&self.boxes[index].get_tooltip_text().unwrap(), evt);
        }
    }

    fn on_image_click(&self, index : usize, evt : &EventButton) {
        if self.images[index].get_visible() {
            self.fire_card_clicked(&self.boxes[index].get_tooltip_text().unwrap(), evt);
        }
    }

    fn init_controls(img_manager : Rc<ImageManager>, row_count : usize, col_count : usize) -> CardView {
        // the easiest way to do this seems to be to create an array of images
        // whose tooltips are the names of their corresponding cards
        // then we can simply set those lying past a certain index
        // to not be visible
        let count = DEFAULT_ROW_COUNT * DEFAULT_COL_COUNT;
        let mut result = CardView{grid : Grid::new(),
            images : Vec::with_capacity(count),
            boxes : Vec::with_capacity(count),
            img_manager : img_manager,
            card_clicked_events : RefCell::new(Vec::new()),
            card_hover_events : RefCell::new(Vec::new()),
            row_count : row_count,
            col_count : col_count};

        for i in 0..result.row_count {
            for j in 0..result.col_count {
                let index = i * result.col_count + j;
                let img = Image::new();
                img.set_visible(false);
                result.images.push(img);
                let evt_box = EventBox::new();
                evt_box.set_above_child(false);
                evt_box.add(&result.images[index]);
                result.boxes.push(evt_box);
                result.grid.attach(&result.boxes[index], j as i32, i as i32, 1, 1);
            }
        }

        result

    }

    pub fn set_cards(&self, cards : &Vec<CardInfo>) {
        // using CardInfos directly removes the need to keep an Rc to the current TCG
        let cutoff = cards.len();
        for i in 0..self.row_count {
            for j in 0..self.col_count {
                let index = i * self.col_count + j;
                if index < cutoff {
                    // TODO: unload previously loaded images
                    self.img_manager.load_image(&cards[index].set_code);
                    if let Some(img) = self.img_manager.get_small_image(&cards[index].set_code) {
                        self.images[index].set_from_pixbuf(Some(&img));
                        self.images[index].set_visible(true);
                        self.boxes[index].set_tooltip_text(Some(&cards[index].name));
                    }
                } else {
                    self.images[index].set_visible(false);
                }
            }
        }
    }

    // TODO: display a single proxy card initially
    // TODO: connect mouse events
    // TODO: update images based upon collection of card names
    // TODO: figure out the best way to have the minimal amount of images loaded at once
    // TODO: create way to handle having more than 20 results
}

