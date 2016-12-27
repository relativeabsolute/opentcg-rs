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
use gtk::{Frame, FlowBox, Orientation, Label};

use open_tcg::game::tcg::TCG;

pub struct CardDisplay {
    pub frame : Frame,
    layout : FlowBox,
    name_label : Label,
    current_tcg : Rc<TCG>
}

impl CardDisplay {
    // TODO: determine if this needs to be an Rc
    // TODO: add card image, description, and parameters
    pub fn new(tcg : Rc<TCG>) -> CardDisplay {
        let instance = CardDisplay{frame : Frame::new(None), layout : FlowBox::new(),
            name_label : Label::new(Some("Card name here")), current_tcg : tcg};

        instance.init_controls();

        instance
    }

    pub fn set_card(&self, name : &String) {
       if let Some(card) = self.current_tcg.cards.get(name) {
           self.name_label.set_text(&card.name);
       }
    }

    fn init_controls(&self) {
        self.layout.set_orientation(Orientation::Vertical);

        //self.name_label.set_text("Card name here");

        self.layout.insert(&self.name_label, -1);

        self.frame.add(&self.layout);
    }
}
