use std::fs;
use std::io::BufReader;
use std::io::Read;

pub mod diff;
pub mod types;

extern crate xmltree;

use std::fs::File;
use xmltree::Element;
use xmltree::XMLNode;

fn build_mesure_attributes(mes: &types::Mesure) -> Element {
    match &mes.attributes {
        None => Element::new("empty"),
        Some(a) => {
            let mut at = Element::new("attributes");
            let mut div = Element::new("divisions");
            let mut time = Element::new("time");
            let mut beats = Element::new("beats");
            let mut beat_type = Element::new("beat-type");
            beats.children.push(XMLNode::Text(a.time.beats.to_string()));
            beat_type
                .children
                .push(XMLNode::Text(a.time.beat_type.to_string()));
            time.children.push(XMLNode::Element(beats));
            time.children.push(XMLNode::Element(beat_type));
            div.children.push(XMLNode::Text(a.divisions.to_string()));
            at.children.push(XMLNode::Element(div));
            let mut key = Element::new("key");
            let mut fifth = Element::new("fifths");
            fifth.children.push(XMLNode::Text(a.key.to_string()));
            key.children.push(XMLNode::Element(fifth));
            at.children.push(XMLNode::Element(key));
            at.children.push(XMLNode::Element(time));
            if let Some(s) = a.staves {
                let mut staves = Element::new("staves");
                staves.children.push(XMLNode::Text(s.to_string()));
                at.children.push(XMLNode::Element(staves));
            }
            for cli in 0..a.clef.len() {
                let mut clef = Element::new("clef");
                let mut sign = Element::new("sign");
                sign.children
                    .push(XMLNode::Text(a.clef[cli].sign.to_string()));
                clef.children.push(XMLNode::Element(sign));
                clef.attributes
                    .insert(String::from("number"), (cli + 1).to_string());
                at.children.push(XMLNode::Element(clef));
            }
            at
        }
    }
}

fn build_mesure(mes: &types::Mesure, number: usize, color: String, att: Element) -> Element {
    let mut mesure = Element::new("measure");
    mesure.attributes.insert(
        String::from("number"),
        std::fmt::format(format_args!("{}", number + 1)),
    );
    mesure.children.push(XMLNode::Element(att));
    for note in mes.notes.iter() {
        if note.typee == "backup" {
            let mut n = Element::new("backup");
            let mut duration = Element::new("duration");
            duration
                .children
                .push(XMLNode::Text(note.duration.to_string()));
            n.children.push(XMLNode::Element(duration));
            mesure.children.push(XMLNode::Element(n));
            continue;
        }
        let mut n = Element::new("note");
        n.attributes.insert(String::from("color"), color.clone());
        if note.is_chord {
            n.children.push(XMLNode::Element(Element::new("chord")));
        }
        match &note.pitch {
            None => n.children.push(XMLNode::Element(Element::new("rest"))),
            Some(p) => {
                let mut pitch = Element::new("pitch");
                let mut step = Element::new("step");
                let mut octave = Element::new("octave");
                step.children.push(XMLNode::Text(p.step.to_string()));
                octave.children.push(XMLNode::Text(p.octave.to_string()));
                pitch.children.push(XMLNode::Element(step));
                if let Some(alt) = p.alter {
                    let mut alter = Element::new("alter");
                    alter.children.push(XMLNode::Text(alt.to_string()));
                    pitch.children.push(XMLNode::Element(alter));
                }
                pitch.children.push(XMLNode::Element(octave));
                n.children.push(XMLNode::Element(pitch));
            }
        };
        let mut duration = Element::new("duration");
        duration
            .children
            .push(XMLNode::Text(note.duration.to_string()));
        n.children.push(XMLNode::Element(duration));
        if note.dot {
            let dot = Element::new("dot");
            n.children.push(XMLNode::Element(dot));
        }
        let mut typee = Element::new("type");
        typee.children.push(XMLNode::Text(note.typee.clone()));
        n.children.push(XMLNode::Element(typee));
        let mut voice = Element::new("voice");
        voice.children.push(XMLNode::Text(note.voice.to_string()));
        n.children.push(XMLNode::Element(voice));
        if let Some(s) = note.staff {
            let mut staff = Element::new("staff");
            staff.children.push(XMLNode::Text(s.to_string()));
            n.children.push(XMLNode::Element(staff));
        }
        mesure.children.push(XMLNode::Element(n));
    }
    mesure
}

fn build_modified_mesure(number: usize, att: Element, notes: &Vec<diff::Diff>) -> Element {
    let mut mesure = Element::new("measure");
    mesure.attributes.insert(
        String::from("number"),
        std::fmt::format(format_args!("{}", number + 1)),
    );
    mesure.children.push(XMLNode::Element(att));
    for note in notes.iter() {
        let mut n = Element::new("note");
        match note {
            diff::Diff::Added(types::Element::note(note)) => {
                n.attributes
                    .insert(String::from("color"), String::from("#69B32B"));
                if note.is_chord {
                    n.children.push(XMLNode::Element(Element::new("chord")));
                }
                match &note.pitch {
                    None => n.children.push(XMLNode::Element(Element::new("rest"))),
                    Some(p) => {
                        let mut pitch = Element::new("pitch");
                        let mut step = Element::new("step");
                        let mut octave = Element::new("octave");
                        step.children.push(XMLNode::Text(p.step.to_string()));
                        octave.children.push(XMLNode::Text(p.octave.to_string()));
                        pitch.children.push(XMLNode::Element(step));
                        if let Some(alt) = p.alter {
                            let mut alter = Element::new("alter");
                            alter.children.push(XMLNode::Text(alt.to_string()));
                            pitch.children.push(XMLNode::Element(alter));
                        }
                        pitch.children.push(XMLNode::Element(octave));
                        n.children.push(XMLNode::Element(pitch));
                    }
                };
                let mut duration = Element::new("duration");
                duration
                    .children
                    .push(XMLNode::Text(note.duration.to_string()));
                n.children.push(XMLNode::Element(duration));
                if note.dot {
                    let dot = Element::new("dot");
                    n.children.push(XMLNode::Element(dot));
                }
                let mut typee = Element::new("type");
                typee.children.push(XMLNode::Text(note.typee.clone()));
                n.children.push(XMLNode::Element(typee));
                let mut voice = Element::new("voice");
                voice.children.push(XMLNode::Text(note.voice.to_string()));
                n.children.push(XMLNode::Element(voice));
                if let Some(s) = note.staff {
                    let mut staff = Element::new("staff");
                    staff.children.push(XMLNode::Text(s.to_string()));
                    n.children.push(XMLNode::Element(staff));
                }
                mesure.children.push(XMLNode::Element(n));
            }
            diff::Diff::Removed(types::Element::note(note)) => {
                n.attributes
                    .insert(String::from("color"), String::from("#F94144"));
                if note.is_chord {
                    n.children.push(XMLNode::Element(Element::new("chord")));
                }
                match &note.pitch {
                    None => n.children.push(XMLNode::Element(Element::new("rest"))),
                    Some(p) => {
                        let mut pitch = Element::new("pitch");
                        let mut step = Element::new("step");
                        let mut octave = Element::new("octave");
                        step.children.push(XMLNode::Text(p.step.to_string()));
                        octave.children.push(XMLNode::Text(p.octave.to_string()));
                        pitch.children.push(XMLNode::Element(step));
                        if let Some(alt) = p.alter {
                            let mut alter = Element::new("alter");
                            alter.children.push(XMLNode::Text(alt.to_string()));
                            pitch.children.push(XMLNode::Element(alter));
                        }
                        pitch.children.push(XMLNode::Element(octave));
                        n.children.push(XMLNode::Element(pitch));
                    }
                };
                let mut duration = Element::new("duration");
                duration
                    .children
                    .push(XMLNode::Text(note.duration.to_string()));
                n.children.push(XMLNode::Element(duration));
                if note.dot {
                    let dot = Element::new("dot");
                    n.children.push(XMLNode::Element(dot));
                }
                let mut typee = Element::new("type");
                typee.children.push(XMLNode::Text(note.typee.clone()));
                n.children.push(XMLNode::Element(typee));
                let mut voice = Element::new("voice");
                voice.children.push(XMLNode::Text(note.voice.to_string()));
                n.children.push(XMLNode::Element(voice));
                if let Some(s) = note.staff {
                    let mut staff = Element::new("staff");
                    staff.children.push(XMLNode::Text(s.to_string()));
                    n.children.push(XMLNode::Element(staff));
                }
                mesure.children.push(XMLNode::Element(n));
            }
            diff::Diff::Unmodified(types::Element::note(note)) => {
                if note.typee == "backup" {
                    let mut b = Element::new("backup");
                    let mut duration = Element::new("duration");
                    duration
                        .children
                        .push(XMLNode::Text(note.duration.to_string()));
                    b.children.push(XMLNode::Element(duration));
                    mesure.children.push(XMLNode::Element(b));
                    continue;
                }
                n.attributes
                    .insert(String::from("color"), String::from("#000000"));
                if note.is_chord {
                    n.children.push(XMLNode::Element(Element::new("chord")));
                }
                match &note.pitch {
                    None => n.children.push(XMLNode::Element(Element::new("rest"))),
                    Some(p) => {
                        let mut pitch = Element::new("pitch");
                        let mut step = Element::new("step");
                        let mut octave = Element::new("octave");
                        step.children.push(XMLNode::Text(p.step.to_string()));
                        octave.children.push(XMLNode::Text(p.octave.to_string()));
                        pitch.children.push(XMLNode::Element(step));
                        if let Some(alt) = p.alter {
                            let mut alter = Element::new("alter");
                            alter.children.push(XMLNode::Text(alt.to_string()));
                            pitch.children.push(XMLNode::Element(alter));
                        }
                        pitch.children.push(XMLNode::Element(octave));
                        n.children.push(XMLNode::Element(pitch));
                    }
                };
                let mut duration = Element::new("duration");
                duration
                    .children
                    .push(XMLNode::Text(note.duration.to_string()));
                n.children.push(XMLNode::Element(duration));
                if note.dot {
                    let dot = Element::new("dot");
                    n.children.push(XMLNode::Element(dot));
                }
                let mut typee = Element::new("type");
                typee.children.push(XMLNode::Text(note.typee.clone()));
                n.children.push(XMLNode::Element(typee));
                let mut voice = Element::new("voice");
                voice.children.push(XMLNode::Text(note.voice.to_string()));
                n.children.push(XMLNode::Element(voice));
                if let Some(s) = note.staff {
                    let mut staff = Element::new("staff");
                    staff.children.push(XMLNode::Text(s.to_string()));
                    n.children.push(XMLNode::Element(staff));
                }
                mesure.children.push(XMLNode::Element(n));
            }
            diff::Diff::Modified(types::Element::note(note), _) => {
                n.attributes
                    .insert(String::from("color"), String::from("#F9C74F"));
                if note.is_chord {
                    n.children.push(XMLNode::Element(Element::new("chord")));
                }
                match &note.pitch {
                    None => n.children.push(XMLNode::Element(Element::new("rest"))),
                    Some(p) => {
                        let mut pitch = Element::new("pitch");
                        let mut step = Element::new("step");
                        let mut octave = Element::new("octave");
                        step.children.push(XMLNode::Text(p.step.to_string()));
                        octave.children.push(XMLNode::Text(p.octave.to_string()));
                        pitch.children.push(XMLNode::Element(step));
                        if let Some(alt) = p.alter {
                            let mut alter = Element::new("alter");
                            alter.children.push(XMLNode::Text(alt.to_string()));
                            pitch.children.push(XMLNode::Element(alter));
                        }
                        pitch.children.push(XMLNode::Element(octave));
                        n.children.push(XMLNode::Element(pitch));
                    }
                };
                let mut duration = Element::new("duration");
                duration
                    .children
                    .push(XMLNode::Text(note.duration.to_string()));
                n.children.push(XMLNode::Element(duration));
                if note.dot {
                    let dot = Element::new("dot");
                    n.children.push(XMLNode::Element(dot));
                }
                let mut typee = Element::new("type");
                typee.children.push(XMLNode::Text(note.typee.clone()));
                n.children.push(XMLNode::Element(typee));
                let mut voice = Element::new("voice");
                voice.children.push(XMLNode::Text(note.voice.to_string()));
                n.children.push(XMLNode::Element(voice));
                if let Some(s) = note.staff {
                    let mut staff = Element::new("staff");
                    staff.children.push(XMLNode::Text(s.to_string()));
                    n.children.push(XMLNode::Element(staff));
                }
                mesure.children.push(XMLNode::Element(n));
            }
            _ => {}
        }
    }
    mesure
}

fn main() {
    let mut args = std::env::args();
    let mut file = File::open(args.nth(1).unwrap()).unwrap();
    let mut file2 = File::open(args.nth(0).unwrap()).unwrap();
    let mut s = String::new();

    file.read_to_string(&mut s).expect("nani?");
    let names_element = Element::parse(s.as_bytes()).unwrap();

    let mut parts: types::ScorePartwise = types::ScorePartwise { parts: Vec::new() };
    for child in names_element.children.iter() {
        let child = child.as_element().expect("Ce n'est pas une node ;-;");
        if child.name == "part" {
            parts.parts.push(types::parsed_to_part(child))
        }
    }

    s = String::new();

    file2.read_to_string(&mut s).expect("Nani?");

    let mut names_element = Element::parse(s.as_bytes()).unwrap();

    let mut parts2: types::ScorePartwise = types::ScorePartwise { parts: Vec::new() };
    for child in names_element.children.iter() {
        let child = child.as_element().expect("Ce n'est pas une node ;-;");
        if child.name == "part" {
            parts2.parts.push(types::parsed_to_part(child))
        }
    }

    let res = diff::diff(&parts, &parts2);

    let clone = names_element.clone();
    for i in 0..clone.children.len() {
        let child = clone.children[i]
            .as_element()
            .expect("Ce n'est pas une node ;-;")
            .clone();
        if child.name == "part" {
            let _ = names_element.take_child(child.name);
        }
    }

    for parts in 0..res.len() {
        let mut part = Element::new("part");
        part.attributes.insert(
            String::from("id"),
            std::fmt::format(format_args!("P{}", parts + 1)),
        );
        for i in 0..res[parts].len() {
            match &res[parts][i] {
                diff::Diff::Added(types::Element::mesure(m)) => {
                    let att = build_mesure_attributes(&m);
                    let mes = build_mesure(&m, i, String::from("#69B32B"), att);
                    part.children.push(XMLNode::Element(mes));
                }
                diff::Diff::Removed(types::Element::mesure(m)) => {
                    let att = build_mesure_attributes(&m);
                    let mes = build_mesure(&m, i, String::from("#F94144"), att);
                    part.children.push(XMLNode::Element(mes));
                }
                diff::Diff::Unmodified(types::Element::mesure(m)) => {
                    let att = build_mesure_attributes(&m);
                    let mes = build_mesure(&m, i, String::from("#000000"), att);
                    part.children.push(XMLNode::Element(mes));
                }
                diff::Diff::Modified(types::Element::mesure(m), diffs) => {
                    let att = build_mesure_attributes(&m);
                    let mes = build_modified_mesure(i, att, &diffs);
                    part.children.push(XMLNode::Element(mes));
                }
                _ => {println!("nani??");}
            }
        }
        names_element.children.push(XMLNode::Element(part));
    }

    let output = File::create("./res.musicxml").unwrap();
    let _ = names_element
        .write(output)
        .expect("Unable to write the file");
}
