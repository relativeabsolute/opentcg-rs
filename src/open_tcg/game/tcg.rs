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

use open_tcg::game::deck::DeckSectionInfo;
use open_tcg::game::card::{CardInfo, CardType};
use open_tcg::util::{files, xml};

use self::sxd_document::QName;
use self::sxd_document::dom::Element;
use std::fmt;

type CardMap = HashMap<String, CardInfo>;
type DeckSections = Vec<DeckSectionInfo>;
type CardTypes = HashMap<String, CardType>;

pub struct TCG {
    name : String,

    // TCGs generally impose a limit on the number of copies
    // of a card that can be in a deck
    card_limit : u32,

    // filename to read card info from
    set_file : String,

    sections : DeckSections,

    cards : CardMap,

    card_types : CardTypes
}

impl fmt::Display for TCG {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl TCG {
    pub fn new() -> TCG {
        TCG{cards : HashMap::new(), name : String::new(),
            card_limit : 0, set_file : String::new(),
            sections : Vec::new(), card_types : HashMap::new()}
    }

    fn read_card_types(directory : &String) -> CardTypes {
        let mut result = HashMap::new();



        result
    }

    fn read_deck(deck_element : &Element) -> DeckSections {
        let mut sections = Vec::new();

        let section_name = QName::new("Section");
        let name_name = QName::new("Name");
        let group_name = QName::new("Group");
        let min_name = QName::new("MinSize");
        let max_name = QName::new("MaxSize");

        for e in deck_element.children() {
            if let Some(element) = e.element() {
                if element.name() == section_name {
                    let mut section = DeckSectionInfo::new();
                    for section_info in element.children() {
                        if let Some(section_element) = section_info.element() {
                            let element_name = section_element.name();
                            if element_name == name_name {
                                section.name = xml::read_text_from_element(&section_element);
                            } else if element_name == group_name {
                                section.group = xml::read_num_from_element(&section_element);
                            } else if element_name == min_name {
                                section.min_size = xml::read_num_from_element(&section_element);
                            } else if element_name == max_name {
                                section.max_size = xml::read_num_from_element(&section_element);
                            }
                        }
                    }
                    sections.push(section);
                }
            }
        }

        sections
    }

    pub fn new_from_file(filename : &String) -> TCG {
        let mut instance = TCG::new();

        let pkg = files::document_from_file(filename);
        let doc = pkg.as_document();
        let root = doc.root();
        let children = root.children();

        let name_name = QName::new("Name");
        let card_limit_name = QName::new("CardLimit");
        let sets_name = QName::new("SetFile");
        let deck_name = QName::new("Deck");
        let types_name = QName::new("TypeDirectory");

        if let Some(tcg_root) = children[0].element() {
            if tcg_root.name() == QName::new("TCG") {
                for e in tcg_root.children() {
                    if let Some(element) = e.element() {
                        let element_name = element.name();
                        if element_name == name_name {
                            instance.name = xml::read_text_from_element(&element);
                        } else if element_name == card_limit_name {
                            instance.card_limit = xml::read_num_from_element(&element);
                        } else if element_name == sets_name {
                            instance.set_file = xml::read_text_from_element(&element);
                        } else if element_name == types_name {
                            let type_dir = xml::read_text_from_element(&element);
                            // TODO: do error checking for empty value
                            instance.card_types = TCG::read_card_types(&type_dir);
                        } else if element_name == deck_name {
                            instance.sections = TCG::read_deck(&element);
                        }
                    }
                }
            }
        }
        // TODO: more stuff here...

        instance
    }
}


