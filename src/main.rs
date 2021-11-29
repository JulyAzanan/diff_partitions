/* /* #[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_xml_rs;

pub mod types;
use types::ScorePartwise; */

extern crate minidom;

use std::fs::File;
use std::io::Read;
use std::fs;
use std::io::BufReader;

fn main() {
    // let mut file = File::open("./tests/1AjoutSameMesure_o.musicxml").unwrap();
    let mut file = File::open("./tests/test.xml").unwrap();
    // let file = BufReader::new(file);
    let mut s = String::new();

    file.read_to_string(&mut s).expect("Nani?!");
    println!("{}", s);
    let root: minidom::Element = s.parse().expect("Fatchouna");
    println!("{:#?}", root);
}
 */

use std::io::Read;
use std::fs;
use std::io::BufReader;

pub mod types;

extern crate xmltree;

use xmltree::Element;
use std::fs::File;

fn main() {
    let mut file = File::open("./tests/1AjoutSameMesure_o.musicxml").unwrap();
    let mut s = String::new();

    file.read_to_string(&mut s).expect("nani?");
    /* let data: &'static str = r##"
    <?xml version="1.0" encoding="utf-8" standalone="yes"?>
    <names>
        <name first="bob" last="jones" />
        <name first="elizabeth" last="smith" />
    </names>
    "##; */
    
    let names_element = Element::parse(s.as_bytes()).unwrap();

    let mut parts: Vec<types::Part> = Vec::new();
    for child in names_element.children.iter() {
        let child = child.as_element().expect("Ce n'est pas une node ;-;");
        if child.name == "part" {
            parts.push(types::parsed_to_part(child))
        }
    }

    
    println!("{:#?}", parts);
}