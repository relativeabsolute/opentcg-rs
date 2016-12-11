extern crate gtk;

use open_tcg::main_window::MainWindow;

mod open_tcg;

use gtk::prelude::*;
use gtk::{Window, WindowType};

fn main() {
	if gtk::init().is_err() {
		println!("Couldn't init GTK.");
		return;
	}
    
    let window = MainWindow::new();
    window.exit_on_close();
    gtk::main();
}
