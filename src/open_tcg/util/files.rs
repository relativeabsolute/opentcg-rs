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

use self::sxd_document::parser;
use self::sxd_document::Package;

use std::io::{BufReader, Lines};
use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;

pub fn lines_from_file(filename : &PathBuf) -> Lines<BufReader<File>> {
    let f = File::open(filename).expect("Error reading file");
    let reader = BufReader::new(f);

    reader.lines()
}

pub fn document_from_file(filename : &PathBuf) -> Package {
    // TODO: add better/more specific error handling here
    let mut s = String::new();
    let mut f = File::open(filename).expect("Error reading file");
    f.read_to_string(&mut s).expect("Error reading file");
 
    parser::parse(&s).expect("error parsing file")
}
