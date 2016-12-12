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

use gtk::prelude::*;
use gtk::{Window, WindowType, Grid, Button};

pub struct MainWindow {
    window : Window
}

impl MainWindow {
    pub fn new() -> MainWindow {
        let window = Window::new(WindowType::Toplevel);
        let instance = MainWindow{window : window};

        instance.determine_size();
        instance.init_controls();

        instance.window.show_all();
        instance
    }

    /// Initializes the layout of the control based on dimensions
    /// determined by the determine_size method.
    fn init_controls(&self) {
        self.window.set_border_width(10);

        let grid = Grid::new();

        let play_button = Button::new_with_label("Play");
        grid.attach(&play_button, 0, 0, 1, 1);
        
        let constructor_button = Button::new_with_label("Deck Constructor");
        grid.attach(&constructor_button, 1, 0, 1, 1);
        
        grid.set_column_spacing(5);

        self.window.add(&grid);
    }

    /// Determines the size and location of the MainWindow based on
    /// the dimensions of the default display.
    /// TODO: fill this in
    fn determine_size(&self) {
        if let Some(scr) = gdk::Screen::get_default() {
            let width = scr.get_width();
            let height = scr.get_height();

            // do some fancy tricks here
            //
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
