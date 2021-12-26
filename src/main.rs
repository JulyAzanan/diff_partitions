use std::io::Read;
use std::fs;
use std::io::BufReader;

pub mod types;
pub mod diff;

extern crate xmltree;

use xmltree::Element;
use std::fs::File;

fn main() {
    let mut file = File::open("./tests/city_of_tears_o.musicxml").unwrap();
    let mut file2 = File::open("./tests/city_of_tears_d.musicxml").unwrap();
    /* let mut file = File::open("./tests/1Ajout1Modification1RetraitSameMesure_o.musicxml").unwrap();
    let mut file2 = File::open("./tests/1Ajout1Modification1RetraitSameMesure_d.musicxml").unwrap(); */
    /* let mut file = File::open("./tests/petit_exemple_o.musicxml").unwrap();
    let mut file2 = File::open("./tests/petit_exemple_d3.musicxml").unwrap(); */
    let mut s = String::new();

    file.read_to_string(&mut s).expect("nani?");
    
    let names_element = Element::parse(s.as_bytes()).unwrap();

    let mut parts: types::ScorePartwise = types::ScorePartwise {parts: Vec::new()};
    for child in names_element.children.iter() {
        let child = child.as_element().expect("Ce n'est pas une node ;-;");
        if child.name == "part" {
            parts.parts.push(types::parsed_to_part(child))
        }
    }


    s = String::new();

    file2.read_to_string(&mut s).expect("Nani?");

    let names_element = Element::parse(s.as_bytes()).unwrap();

    let mut parts2: types::ScorePartwise = types::ScorePartwise {parts: Vec::new()};
    for child in names_element.children.iter() {
        let child = child.as_element().expect("Ce n'est pas une node ;-;");
        if child.name == "part" {
            parts2.parts.push(types::parsed_to_part(child))
        }
    }

    
    // println!("{:#?}", parts2);
    let res = diff::diff(&parts, &parts2);
    println!("{:#?}", res);
}