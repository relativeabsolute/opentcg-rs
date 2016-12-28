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
use gtk::{TreeView, ListStore, TreeViewColumn, CellRendererPixbuf};
use self::gdk_pixbuf::Pixbuf;

pub struct CardView {
    pub view : TreeView,
    pub model : ListStore
}

impl CardView {
    pub fn new() -> CardView {
        let view = TreeView::new();

        let pixbuf_type = Pixbuf::static_type();

        // TODO: figure out how to add more columns
        const column_count : usize = 1;
        let mut columns = [pixbuf_type; column_count];

        // TODO: finish the model for a grid of card images

        let store = ListStore::new(&columns);

        let renderer = CellRendererPixbuf::new();

        let col = TreeViewColumn::new();
        for i in 0..column_count {
            col.pack_start(&renderer, false);
            col.add_attribute(&renderer, "pixbuf", i as i32);
        }

        view.set_headers_visible(false);
        view.set_model(Some(&store));

        CardView{view : view, model : store}
    }
}

