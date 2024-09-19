const NOTES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

#[derive(Clone, Copy)]
pub enum Interval {
    Whole,
    Half,
}

impl Interval {
    pub fn unit(self) -> usize {
        match self {
            Interval::Half => 1,
            Interval::Whole => 2,
        }
    }
}

const MAJOR_FORMULAR: [Interval; 6] = [
    Interval::Whole,
    Interval::Whole,
    Interval::Half,
    Interval::Whole,
    Interval::Whole,
    Interval::Whole,
];

pub fn get_major_notes(key: usize) {
    let mut corsor_idx = key;

    let mut notes: Vec<String> = Vec::new();
    notes.push(NOTES[corsor_idx].into());

    for i in 0..6 {
        let interval = MAJOR_FORMULAR[i].clone().unit();
        corsor_idx += interval;

        if corsor_idx > 11 {
            corsor_idx -= 12;
        }

        notes.push(String::from(NOTES[corsor_idx as usize]));
    }

    println!("{}", String::from(NOTES[key]));
    println!("{:?}", notes);
}
