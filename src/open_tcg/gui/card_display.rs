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
use gtk::{Builder, Frame, Image, TextView, Label, WrapMode};

use open_tcg::game::tcg::TCG;
use super::image_manager::ImageManager;

pub struct CardDisplay {
    pub frame : Frame,
    card_image : Image,
    card_name_label : Label,
    card_text_view : TextView,
    current_tcg : Rc<TCG>,
    img_manager : Rc<ImageManager>
}

impl CardDisplay {
    // TODO: determine if this needs to be an Rc
    // TODO: add card image, description, and parameters
    pub fn new(tcg : Rc<TCG>, img_manager : Rc<ImageManager>) -> CardDisplay {
        let glade_src = include_str!("card_display.glade");
        let builder = Builder::new_from_string(glade_src);

        let instance = CardDisplay{frame : builder.get_object("card_display").unwrap(),
            card_name_label : builder.get_object("card_name_label").unwrap(),
            card_text_view : builder.get_object("card_text_view").unwrap(),
            card_image : builder.get_object("card_image").unwrap(), current_tcg : tcg,
            img_manager : img_manager};

        // TODO: prevent the ugly, constant resizing upon changing the card
        instance.card_text_view.set_wrap_mode(WrapMode::Word);

        instance
    }

    pub fn set_card(&self, name : &String) {
       if let Some(card) = self.current_tcg.cards.get(name) {
           self.card_name_label.set_text(&card.name);
           if let Some(buffer) = self.card_text_view.get_buffer() {
               buffer.set_text(&card.text);
           }
           if let Some(img) = self.img_manager.get_large_image(&card.set_code) {
               self.card_image.set_from_pixbuf(Some(&img));
           }
       }
    }
}
