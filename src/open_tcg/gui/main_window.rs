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
use std::path::PathBuf;

use gtk::prelude::*;
use gtk::{Builder, Window, WindowType, Grid, Button};

use open_tcg::gui::deck_editor::DeckEditor;
use open_tcg::game::tcg::TCG;

pub struct MainWindow {
    window : Window,
    play_button : Button,
    deck_edit_button : Button,
    view_profile_button : Button,
    current_tcg : Rc<TCG>
}

impl MainWindow {
    pub fn new() -> Rc<MainWindow> {
        // TODO: read settings from file for default TCGs directory
        let path = PathBuf::from("example.xml");
        let tcg = Rc::new(TCG::new_from_file(&path));
        let instance = Rc::new(MainWindow::init_controls(tcg));
        
        instance.determine_size();
        MainWindow::connect_events(instance.clone());
        

        instance.window.show_all();
        instance
    }

    fn init_controls(tcg : Rc<TCG>) -> MainWindow {
        let mut instance = MainWindow{window : Window::new(WindowType::Toplevel),
            play_button : Button::new(), deck_edit_button : Button::new(),
            view_profile_button : Button::new(), current_tcg : tcg};

        let glade_src = include_str!("main_window.glade");
        println!("{}", glade_src);
        let builder = Builder::new_from_string(glade_src);

        instance.window = builder.get_object("window").unwrap();
        instance.play_button = builder.get_object("play_button").unwrap();
        instance.deck_edit_button = builder.get_object("deck_edit_button").unwrap();
        instance.view_profile_button = builder.get_object("view_profile_button").unwrap();

        instance
    }

    fn connect_events(instance : Rc<MainWindow>) {
        // attach events here
        {
            let instance_copy = instance.clone();
            instance.play_button.connect_clicked(move |widget| {
                instance_copy.on_play_clicked();
            });
        }
        {
            let instance_copy = instance.clone();
            instance.deck_edit_button.connect_clicked(move |widget| {
                instance_copy.on_constructor_clicked();
            });
        }
        {
            let instance_copy = instance.clone();
            instance.view_profile_button.connect_clicked(move |_| {
                instance_copy.on_view_profile_clicked();
            });
        }
    }

    fn on_view_profile_clicked(&self) {

    }

    fn on_play_clicked(&self) {
        // TODO: display interface to enter rated pool/challenge a friend/spectate
    }

    fn on_constructor_clicked(&self) {
        // TODO: display deck editor
        // for now construct a new instance each time (should probably change later)
        let editor = DeckEditor::new(self.current_tcg.clone());
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
