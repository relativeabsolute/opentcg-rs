extern crate gtk;

use gtk::prelude::*;
use gtk::{Window, WindowType};

pub struct MainWindow {
    window : gtk::Window
}

impl MainWindow {
    pub fn new() -> MainWindow {
        let window = Window::new(WindowType::Toplevel);
        let instance = MainWindow{window : window};

        instance.init_controls();

        instance.window.show_all();
        instance
    }

    fn init_controls(&self) {
        // add stuff here
    }

    pub fn exit_on_close(&self) {
        self.window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(true)
        });
    }
}
