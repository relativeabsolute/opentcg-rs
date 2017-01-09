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

use std::cell::RefCell;

use super::card::CardInfo;
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

use self::sxd_document::Package;
use self::sxd_document::writer::format_document;

/// This structure defines an abstraction of the information
/// associated with a subsection of a deck, such as main, side, etc.
#[derive(Debug, Clone)]
pub struct DeckSectionInfo {
    pub name : String,
    pub group : u32,

    /// Defines the minimum number of cards in this section of the deck
    pub min_size : u32,

    /// Defines the maximum number of cards in this section of the deck
    pub max_size : u32,

    /// Defines the number of rows to be used in the deck editor for this section
    pub rows : u32,

    /// Defines the number of columns to be used in the deck editor for this section
    pub columns : u32
}

impl DeckSectionInfo {
    pub fn new() -> DeckSectionInfo {
        DeckSectionInfo{name : String::new(), group : 0, min_size : 0, max_size : 0,
            rows : 0, columns : 0}
    }
}

pub struct DeckSection {
    /// Descriptor of the meta data associated with this deck section.
    pub info : DeckSectionInfo,

    /// Defines a map from card name to number of copies in this section
    pub cards : RefCell<HashMap<String, u32>>
}

impl DeckSection {
    pub fn new() -> DeckSection {
        DeckSection { info : DeckSectionInfo::new(), cards : RefCell::new(HashMap::new()) }
    }

    pub fn new_from_file() -> DeckSection {
        let mut result = DeckSection::new();

        // TODO: fill this in

        result
    }

    pub fn write_to_file(&self, filename : &PathBuf) {
        let package = Package::new();
        let doc = package.as_document();

        let section = doc.create_element("Section");

        let section_name = doc.create_element("Name");
        let section_name_text = doc.create_text(&self.info.name);
        section_name.append_child(section_name_text);
        section.append_child(section_name);

        let section_cards = doc.create_element("Cards");
        for (name, copies) in self.cards.borrow().iter() {
            let card = doc.create_element("Card");
            let card_name = doc.create_element("Name");
            let card_name_text = doc.create_text(&name);
            card_name.append_child(card_name_text);
            card.append_child(card_name);

            let card_copies = doc.create_element("NumCopies");
            let card_copies_text = doc.create_text(&format!("{}", copies));
            card_copies.append_child(card_copies_text);
            card.append_child(card_copies);

            section_cards.append_child(card);
        }
        section.append_child(section_cards);

        doc.root().append_child(section);

        let mut file = File::create(filename).expect("Error writing file");
        format_document(&doc, &mut file).ok().expect("Error writing document");
    }
}

pub struct Deck {
    pub sections : Vec<DeckSection>,
    pub name : String
}
