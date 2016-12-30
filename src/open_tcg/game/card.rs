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

extern crate sxd_document;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

use self::sxd_document::QName;
use self::sxd_document::dom::Element;

use open_tcg::util::{files, xml};

type ParamValues = HashMap<String, i32>;

// Note: this represents the info associated with a card common to editing and game play.
// There will be a different structure representing attributes of a card in game (such as location,
// orientation, face-up/face-down, etc.)
#[derive(Debug, Clone)]
pub struct CardInfo {
    pub name : String,
    pub card_type : CardType,
    pub param_values : ParamValues,
    pub set_name : String,
    pub set_code : String // this is also used as an image handle
    // TODO: fill this in with card type, parameters, etc.
    
}

impl CardInfo {
    pub fn new() -> CardInfo {
        CardInfo{name : String::new(), card_type : CardType::new(), param_values : HashMap::new(),
            set_name : String::new(), set_code : String::new()}
    }

    pub fn new_from_file(filename : &PathBuf) -> CardInfo {
        let mut result = CardInfo::new();

        let pkg = files::document_from_file(filename);
        let doc = pkg.as_document();
        let children = doc.root().children();

        let name_name = QName::new("Name");
        let set_code_name = QName::new("SetCode");
        let set_name_name = QName::new("SetName");

        if let Some(card_root) = children[0].element() {
            for e in card_root.children() {
                if let Some(element) = e.element() {
                    let element_name = element.name();
                    if element_name == name_name {
                        result.name = xml::read_text_from_element(&element);
                    } else if element_name == set_code_name {
                        result.set_code = xml::read_text_from_element(&element);
                    } else if element_name == set_name_name {
                        result.set_name = xml::read_text_from_element(&element);
                    }
                }
            }
        }

        result
    }
}

#[derive(Debug, Clone)]
pub struct CardType {
    pub name : String,
    pub param_names : Vec<String>
}

impl CardType {
    pub fn new() -> CardType {
        CardType{name : String::new(), param_names : Vec::new()}
    }

    pub fn new_from_file(filename : &PathBuf) -> CardType {
        let mut result = CardType::new();

        let pkg = files::document_from_file(filename);
        let doc = pkg.as_document();
        let children = doc.root().children();

        let name_name = QName::new("Name");
        let params_name = QName::new("Parameters");
        // TODO: figure out what to do with aliases/subtypes

        if let Some(type_root) = children[0].element() {
            if type_root.name() == QName::new("CardType") {
                for e in type_root.children() {
                    if let Some(element) = e.element() {
                        let element_name = element.name();
                        if element_name == name_name {
                            result.name = xml::read_text_from_element(&element);
                        } else if element_name == params_name {
                            for param in element.children() {
                                if let Some(param_elem) = param.element() {
                                    result.param_names.push(xml::read_text_from_element(&param_elem));
                                }
                            }
                        }
                    }
                }
            }
        }

        result
    }
}
