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
use gtk::prelude::*;
use gtk::{Window, WindowType, Grid, Button};

use open_tcg::deck_editor::DeckEditor;
use open_tcg::game::tcg::TCG;

pub struct MainWindow {
    window : Window,
    grid : Grid,
    play_button : Button,
    constructor_button : Button
}

impl MainWindow {
    pub fn new() -> Rc<MainWindow> {
        let window = Window::new(WindowType::Toplevel);
        let instance = Rc::new(MainWindow{window : window,
            play_button : Button::new_with_label("Play"),
            constructor_button : Button::new_with_label("Deck Constructor"),
            grid : Grid::new()});
        
        instance.determine_size();
        instance.init_controls();
        
        // attach events here
        // TODO: move attachments into its own function
        {
            let instance_copy = instance.clone();
            instance.play_button.connect_clicked(move |widget| {
                instance_copy.on_play_clicked();
            });
        }
        {
            let instance_copy = instance.clone();
            instance.constructor_button.connect_clicked(move |widget| {
                instance_copy.on_constructor_clicked();
            });
        }


        instance.window.show_all();
        instance
    }

    fn on_play_clicked(&self) {
        // TODO: display interface to enter rated pool/challenge a friend/spectate
    }

    fn on_constructor_clicked(&self) {
        // TODO: display deck editor
        // for now construct a new instance each time (should probably change later)
        let editor = DeckEditor::new();
    }

    /// Initializes the layout of the control based on dimensions
    /// determined by the determine_size method.
    fn init_controls(&self) {
        self.window.set_border_width(10);

        self.grid.attach(&self.play_button, 0, 0, 1, 1);
        
        self.grid.attach(&self.constructor_button, 1, 0, 1, 1);
        
        self.grid.set_column_spacing(5);

        self.window.add(&self.grid);
    }

    /// Determines the size and location of the MainWindow based on
    /// the dimensions of the default display.
    fn determine_size(&self) {
        if let Some(scr) = gdk::Screen::get_default() {
            let width = scr.get_width() as f64;
            let height = scr.get_height() as f64;
            
            let new_width = (width * 0.75) as i32;
            let new_height = (height * 0.75) as i32;

            self.window.set_default_size(new_width, new_height);
            self.window.set_position(gtk::WindowPosition::Center);
        }
    }

    /// Exit the application when the user closes the window
    pub fn exit_on_close(&self) {
        self.window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(true)
        });
    }
}
