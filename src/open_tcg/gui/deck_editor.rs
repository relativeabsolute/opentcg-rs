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
use gtk::{Window, WindowType, FlowBox, Button, Orientation};

use open_tcg::game::tcg::TCG;
use super::card_display::CardDisplay;

pub struct DeckEditor {
    window : Window,
    layout : FlowBox,
    card_display : CardDisplay,
    // TODO: add controls here
    current_tcg : Rc<TCG>
}

impl DeckEditor {
    pub fn new(tcg : Rc<TCG>) -> Rc<DeckEditor> {
        let window = Window::new(WindowType::Toplevel);
        let instance = Rc::new(DeckEditor{window : window, layout : FlowBox::new(),
            card_display : CardDisplay::new(tcg.clone()), current_tcg : tcg});

        instance.init_controls();

        // TODO: fill in control setup and event connections

        instance.window.show_all();
        instance
    }

    fn init_controls(&self) {
        // TODO: add card inspection, deck view, and card search
        // TODO: also auxiliary functions (import/export)
        self.layout.set_orientation(Orientation::Horizontal);

        // add card display, deck section views, and card search
        self.layout.insert(&self.card_display.frame, -1);

        self.window.add(&self.layout);
    }
}
