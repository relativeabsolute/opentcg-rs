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

const row_count : usize = 5;
const col_count : usize = 4;
const num_images : row_count * col_count;

pub struct CardView {
    pub grid : Grid,
    images : Vec<Image>,
    boxes : Vec<EventBox>,
    current_tcg : Rc<TCG>
}

impl CardView {
    pub fn new(tcg : Rc<TCG>) -> Rc<CardView> {
        let instance = Rc::new(CardView::init_controls(tcg));



        instance
    }

    fn init_controls(tcg : Rc<TCG>) -> CardView {
        // the easiest way to do this seems to be to create an array of images
        // whose tooltips are the names of their corresponding cards
        // then we can simply set those lying past a certain index
        // to not be visible
        let mut result = CardView{grid : Grid::new(),
            images : Vec::with_capacity(row_count * col_count),
            boxes : Vec::with_capacity(row_count * col_count),
            current_tcg : tcg};

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

    pub fn set_cards(&self, card_names : Vec<String>) {
        let cards : Vec<&CardInfo> = card_names.iter().map(|n| self.current_tcg.cards.get(n).unwrap()).collect();
        let cutoff = cards.len();
        for i in 0..num_images {
            if i < cutoff {
                // TODO: load the corresponding image and make it visible
            } else {
                // TODO: make the corresponding image invisible
            }
        }
    }

    // TODO: display a single proxy card initially
    // TODO: connect mouse events
    // TODO: update images based upon collection of card names
    // TODO: figure out the best way to have the minimal amount of images loaded at once
    // TODO: create way to handle having more than 20 results
}

