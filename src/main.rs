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

use open_tcg::main_window::MainWindow;
use open_tcg::game::tcg::TCG;

mod open_tcg;

use gtk::prelude::*;
use gtk::{Window, WindowType};
use std::env;

fn main() {
	if gtk::init().is_err() {
		println!("Couldn't init GTK.");
		return;
	}
    
    if let Ok(x) = env::current_dir() {
        println!("Current directory is {}", x.display());
    }
    let tcg = TCG::new_from_file(&"yugioh.xml".to_string());
    
    // this is the window that allows navigation
    let window = MainWindow::new();
    window.exit_on_close();
    gtk::main();
}
