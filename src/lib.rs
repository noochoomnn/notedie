fn normalize_distance(mut distance: usize) -> usize {
    if distance > 11 {
        distance = distance % 12;
    }
    distance
}

enum Step {
    None,
    Whole,
    Half,
}

impl Step {
    pub fn usize(&self) -> usize {
        match self {
            Step::None => 0,
            Step::Half => 1,
            Step::Whole => 2,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Accidental {
    Natural,
    Flat,
    Sharp,
}

const SHARP_NOTES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

const FLAT_NOTES: [&str; 12] = [
    "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B",
];

struct Note {
    str: String,
    index: usize,
}

impl Note {
    pub fn from_str(str: &str) -> Note {
        Note {
            str: String::from(str),
            index: Note::index_from_str(str),
        }
    }

    pub fn from_distance(root: &Note, distance: &usize, accidental: &Accidental) -> Note {
        let dt = root.index() + distance;
        let index = normalize_distance(dt);
        let str = match accidental {
            Accidental::Natural | Accidental::Sharp => String::from(SHARP_NOTES[index]),
            Accidental::Flat => String::from(FLAT_NOTES[index]),
        };

        Note { str, index }
    }

    pub fn to_string(&self) -> String {
        self.str.clone()
    }

    fn index_from_str(str: &str) -> usize {
        match str {
            "C" => 0,
            "C#" => 1,
            "Db" => 1,
            "D" => 2,
            "D#" => 3,
            "Eb" => 3,
            "E" => 4,
            "F" => 5,
            "F#" => 6,
            "Gb" => 6,
            "G" => 7,
            "G#" => 8,
            "Ab" => 8,
            "A" => 9,
            "A#" => 10,
            "Bb" => 10,
            "B" => 11,
            _ => 0,
        }
    }
    fn index(&self) -> usize {
        self.index.clone()
    }
}

const MAJOR_STRUCTURE: [Step; 7] = [
    Step::None,
    Step::Whole,
    Step::Whole,
    Step::Half,
    Step::Whole,
    Step::Whole,
    Step::Whole,
];

const NATURAL_MINOR_STRUCTURE: [Step; 7] = [
    Step::None,
    Step::Whole,
    Step::Half,
    Step::Whole,
    Step::Whole,
    Step::Half,
    Step::Whole,
];

const MAJOR7_STRUCTURE: [usize; 4] = [1, 3, 5, 7];

fn half_steps_from_scale(scale: &str) -> Vec<usize> {
    let mut half_steps: Vec<usize> = Vec::new();
    let scale_ref = match scale {
        "major" => MAJOR_STRUCTURE,
        "minor" => NATURAL_MINOR_STRUCTURE,
        _ => MAJOR_STRUCTURE,
    };

    let mut current_step: usize = 0;
    for step in scale_ref.iter() {
        current_step += step.usize();
        half_steps.push(current_step.clone());
    }

    half_steps
}

fn get_note_from_scale(scale: &str, root: &str, accidental: Accidental) -> Vec<String> {
    let root_note = Note::from_str(root);

    let half_steps = half_steps_from_scale(scale);
    let mut notes: Vec<String> = Vec::new();

    for step in half_steps.iter() {
        let note = Note::from_distance(&root_note, step, &accidental);

        notes.push(note.to_string());
    }

    notes
}

fn get_note_from_chord(chord_type: &str, root: &str) -> Vec<String> {
    let root_note = Note::from_str(root);

    let half_steps = half_steps_from_scale("major");
    let mut notes: Vec<String> = Vec::new();

    for step in MAJOR7_STRUCTURE.iter() {
        let distance = half_steps[step - 1];
        let note = Note::from_distance(&root_note, &distance, &Accidental::Natural);

        notes.push(note.to_string());
    }

    notes
}

pub fn major_scale(root: &str, accidental: Accidental) -> Vec<String> {
    get_note_from_scale("major", root, accidental)
}

pub fn natural_minor_scale(root: &str, accidental: Accidental) -> Vec<String> {
    get_note_from_scale("minor", root, accidental)
}

pub fn major7(root: &str) -> Vec<String> {
    get_note_from_chord("major7", root)
}
