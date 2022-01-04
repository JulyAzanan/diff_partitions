#[derive(Debug, PartialEq, Clone)]
pub struct Pitch {
    pub step: char,
    pub octave: u8,
    pub alter: Option<i8>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Note {
    pub pitch: Option<Pitch>, //Pitch(Step, octave, duration, type)
    pub duration: u8, //Silence(duration, type)
    pub typee: String,
    pub voice: u8,
    pub staff: Option<u8>,
    pub dot: bool,
    pub is_chord: bool
}

#[derive(Debug, PartialEq, Clone)]
pub struct Clef {
    pub sign: char,
}

#[derive(Debug, Clone)]
pub struct Part {
    pub measures: Vec<Mesure>,
}

#[derive(Debug)]
pub struct ScorePartwise {
    pub parts: Vec<Part>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Attributes {
    pub divisions: u8, //unité de division de la noire le plus petit apparaissant dans la partition entière (1 pour une partition avec que des noires, 2 pour une contenant aussi des croches, etc)
    pub clef: Vec<Clef>,    //Clé de la portée
    pub time: Time,
    pub staves: Option<u8>,
    pub key: i8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Time {
    pub beats: u8, //nombre du haut dans l'indication de mesure
    pub beat_type: u8, //nombre du bas dans l'indication de mesure
}

#[derive(Debug, PartialEq, Clone)]
pub struct Mesure {
    pub attributes: Option<Attributes>,
    pub notes: Vec<Note>, //Ensemble des notes et silences contenus dans la mesure
}

#[derive(Debug)]
pub enum Element {
    pitch(Pitch), 
    note(Note), 
    clef(Clef),
    part(Part),
    attributes(Attributes), 
    time(Time),
    mesure(Mesure), 
}

impl Mesure {
    pub fn print_mesure(&self) -> () {
        println!("Mesure : {:?}", self);
        for note in self.notes.iter() {
            println!("{:?}", note)
        }
    }
}

impl ScorePartwise {
    pub fn print_score(&self) -> () {
        println!("Score : {:?}", self);
        for part in self.parts.iter() {
            println!("Part : {:?}", part);
            for measure in part.measures.iter() {
                measure.print_mesure();
            }
        }
    }
}

pub fn parsed_to_part(parsed: &xmltree::Element) -> Part {
    let mut res = Part {measures: Vec::new()};
    for measure in parsed.children.iter() {
        let measure = measure.as_element().expect("Ce n'est pas une mesure ;-;");
        res.measures.push(parsed_to_measure(measure))
    }
    res
}

pub fn parsed_to_measure(parsed: &xmltree::Element) -> Mesure {
    let mut res = Mesure {
        attributes: None,
        notes: Vec::new(),
    };
    for child in parsed.children.iter() {
        let child = child.as_element().expect("Ce n'est pas une node ;-;");
        if child.name == "attributes" {
            res.attributes = Some(parsed_to_attributes(child))
        }
        if child.name == "note" {
            res.notes.push(parsed_to_note(child))
        }
        if child.name == "backup" {
            res.notes.push(parsed_to_backup(child));
        }
    }
    res
}

pub fn parsed_to_attributes(parsed: &xmltree::Element) -> Attributes {
    /* let clef = Clef {
        sign: 'a'
    }; */
    let time = Time {
        beats: 0,
        beat_type: 0
    };
    let mut res = Attributes {
        divisions: 0,
        clef: Vec::new(),
        time: time,
        staves: None,
        key: 0
    };
    for child in parsed.children.iter() {
        let child = child.as_element().expect("Ce n'est pas une node ;-;");
        if child.name == "divisions" {
            res.divisions = string_to_u8(child.children[0].as_text().expect("C'est pas un text wesh"));
        }
        if child.name == "time" {
            res.time = parsed_to_time(child)
        }
        if child.name == "clef" {
            res.clef.push(parsed_to_clef(child))
        }
        if child.name == "staves" {
            res.staves = Some(string_to_u8(child.children[0].as_text().expect("C'est pas un text wesh")));
        }
        if child.name == "key" {
            res.key = string_to_i8(child.children[0].as_element().unwrap().children[0].as_text().expect("C'est pas un text wesh"));
        }
    }
    res
}

pub fn parsed_to_clef(parsed: &xmltree::Element) -> Clef {
    let mut res = Clef {
        sign: 'a'
    };
    for child in parsed.children.iter() {
        let child = child.as_element().expect("Ce n'est pas une node ;-;");
        if child.name == "sign" {
            res.sign = child.children[0].as_text().expect("C'est pas un text wesh").chars().nth(0).expect("String vide");
        }
    }
    res
}

pub fn parsed_to_time(parsed: &xmltree::Element) -> Time {
    let mut res = Time {
        beats: 0,
        beat_type: 0
    };
    for child in parsed.children.iter() {
        let child = child.as_element().expect("Ce n'est pas une node ;-;");
        if child.name == "beats" {
            res.beats = string_to_u8(child.children[0].as_text().expect("C'est pas un text wesh"))
        }
        if child.name == "beat-type" {
            res.beat_type = string_to_u8(child.children[0].as_text().expect("C'est pas un text wesh"))
        }
    }
    res
}

pub fn parsed_to_note(parsed: &xmltree::Element) -> Note {
    let mut res = Note {
        duration: 0,
        typee: String::new(),
        pitch: None,
        voice: 1,
        staff: None,
        dot: false,
        is_chord: false
    };
    let mut is_silence = false;
    for child in parsed.children.iter() {
        let child = child.as_element().expect("Ce n'est pas une node ;-;");
        if child.name == "duration" {
            res.duration = string_to_u8(child.children[0].as_text().expect("C'est pas un text wesh"))
        }
        if child.name == "type" {
            res.typee = child.children[0].as_text().expect("C'est pas un text wesh").to_string();
        }
        if child.name == "rest" {
            is_silence = true;
        }
        if child.name == "voice" {
            res.voice = string_to_u8(child.children[0].as_text().expect("C'est pas un text wesh"));
        }
        if child.name == "staff" {
            res.staff = Some(string_to_u8(child.children[0].as_text().expect("C'est pas un text wesh")));
        }
        if child.name == "dot" {
            res.dot = true;
        }
        if child.name == "chord" {
            res.is_chord = true;
        }
        if child.name == "pitch" {
            res.pitch = if is_silence {
                None
            } else {
                Some(parsed_to_pitch(child))
            }
        }
    }
    res
}

pub fn parsed_to_pitch(parsed: &xmltree::Element) -> Pitch {
    let mut res = Pitch {
        octave: 0,
        step: 'a',
        alter: None
    };
    for child in parsed.children.iter() {
        let child = child.as_element().expect("Ce n'est pas une node ;-;");
        if child.name == "octave" {
            res.octave = string_to_u8(child.children[0].as_text().expect("C'est pas un text wesh"))
        }
        if child.name == "step" {
            res.step = child.children[0].as_text().expect("C'est pas un text wesh").chars().nth(0).expect("string vide");
        }
        if child.name == "alter" {
            res.alter = Some(string_to_i8(child.children[0].as_text().expect("C'est pas un text wesh")));
        }
    }
    res
}

pub fn parsed_to_backup(parsed: &xmltree::Element) -> Note {
    let mut res = Note {
        duration: 0,
        typee: String::from("backup"),
        pitch: None,
        voice: 1,
        staff: None,
        dot: false,
        is_chord: false
    };
    for child in parsed.children.iter() {
        let child = child.as_element().expect("Ce n'est pas une node ;-;");
        if child.name == "duration" {
            res.duration = string_to_u8(child.children[0].as_text().expect("C'est pas un text wesh"));
        }
    };
    res
}

fn string_to_u8(s: &str) -> u8 {
    let res : u8 = s.parse().unwrap();
    res
}

fn string_to_i8(s: &str) -> i8 {
    let res : i8 = s.parse().unwrap();
    res
}