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

use gtk::prelude::*;
use gtk::{Grid, EventBox, Image};
use self::gdk_pixbuf::Pixbuf;

use open_tcg::game::tcg::TCG;
use open_tcg::game::card::CardInfo;
use super::image_manager::ImageManager;

const row_count : usize = 5;
const col_count : usize = 4;

pub struct CardView {
    pub grid : Grid,
    images : Vec<Image>,
    boxes : Vec<EventBox>,
    img_manager : Rc<ImageManager>
}

impl CardView {
    pub fn new(img_manager : Rc<ImageManager>) -> Rc<CardView> {
        let instance = Rc::new(CardView::init_controls(img_manager));



        instance
    }

    fn init_controls(img_manager : Rc<ImageManager>) -> CardView {
        // the easiest way to do this seems to be to create an array of images
        // whose tooltips are the names of their corresponding cards
        // then we can simply set those lying past a certain index
        // to not be visible
        let mut result = CardView{grid : Grid::new(),
            images : Vec::with_capacity(row_count * col_count),
            boxes : Vec::with_capacity(row_count * col_count),
            img_manager : img_manager};

        for i in 0..row_count {
            for j in 0..col_count {
                let index = i * col_count + j;
                let img = Image::new();
                img.set_visible(false);
                result.images.push(img);
                let evt_box = EventBox::new();
                evt_box.add(&result.images[index]);
                result.boxes.push(evt_box);
                result.grid.attach(&result.boxes[index], j as i32, i as i32, 1, 1);
            }
        }

        result

    }

    pub fn set_cards(&self, cards : &Vec<&CardInfo>) {
        // using CardInfos directly removes the need to keep an Rc to the current TCG
        let cutoff = cards.len();
        for i in 0..row_count {
            for j in 0..col_count {
                let index = i * col_count + j;
                if index < cutoff {
                    // TODO: unload previously loaded images
                    self.img_manager.load_image(&cards[index].set_code);
                    if let Some(img) = self.img_manager.get_small_image(&cards[index].set_code) {
                        self.images[index].set_from_pixbuf(Some(&img));
                        self.images[index].set_visible(true);
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

