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
use std::cell::RefCell;

use self::gdk_pixbuf::Pixbuf;
use std::collections::HashMap;

struct ImageSizes {
    pub small : Rc<Pixbuf>,
    pub large : Rc<Pixbuf>
}

const DEFAULT_SMALL_SCALE : i32 = 100;
const DEFAULT_LARGE_SCALE : i32 = 250;

pub struct ImageManager {
    images : RefCell<HashMap<String, Rc<ImageSizes>>>,
    pub small_scale : i32,
    pub large_scale : i32
}

impl ImageManager {
    pub fn new() -> ImageManager {
        ImageManager{images : RefCell::new(HashMap::new()),
            small_scale : DEFAULT_SMALL_SCALE,
            large_scale : DEFAULT_LARGE_SCALE}
    }

    pub fn unload_image(&self, set_code : &String) {
        self.images.borrow_mut().remove(set_code);
    }

    pub fn load_image(&self, set_code : &String) {
        let mut images = self.images.borrow_mut();
        if !images.contains_key(set_code) {
            let filename = "images/".to_string() + set_code + ".png";
            if let Ok(small) = Pixbuf::new_from_file_at_size(&filename,
                self.small_scale, self.small_scale) {
                if let Ok(large) = Pixbuf::new_from_file_at_size(&filename,
                    self.large_scale, self.large_scale) {
                    let result = ImageSizes{small : Rc::new(small), large : Rc::new(large)};
                    images.insert(set_code.clone(), Rc::new(result));
                }
            }
        }
    }

    pub fn get_small_image(&self, set_code : &String) -> Option<Rc<Pixbuf>> {
        let images = self.images.borrow();
        match images.get(set_code) {
            Some(ref img) => Some(img.small.clone()),
            None => None
        }
    }

    pub fn get_large_image(&self, set_code : &String) -> Option<Rc<Pixbuf>> {
        let images = self.images.borrow();
        match images.get(set_code) {
            Some(ref img) => Some(img.large.clone()),
            None => None
        }
    }
}
